mod banner;
pub mod boot;
pub(crate) mod bootings;
mod compression;
mod graceful;
mod parse;

#[cfg(feature = "aws-lambda")]
use bootings::aws_lambda::LambdaRequestEvent;

#[cfg(feature = "tls")]
use bootings::tls;
use hyper_util::rt::TokioExecutor;
use tokio::time::error::Elapsed;
use tokio::time::Instant;

use crate::application_properties;
use crate::application_properties::CompressionAlgorithm;
use crate::di;
use crate::header;
pub(crate) mod route;

use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper::StatusCode;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use log::Level;
use tokio::net::TcpListener;

use crate::header::preprocess_headers;
use crate::logger::print_system_log;
use crate::swagger::context::SwaggerContext;
use crate::IModule;

pub async fn run_server(
    application_properties: application_properties::ApplicationProperties,
    root_module: impl IModule + Clone + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    // 1. DI Context Initialize
    let mut di_context = di::DIContext::new();
    di_context.initialize(Box::new(root_module.clone()));
    let di_context = Arc::new(di_context);

    // 2. Prepare Swagger Serving, if enabled
    if let Some(swagger_context) = di_context.get::<SwaggerContext>() {
        swagger_context.initialize_from_module(root_module.clone());
    }

    // 3. ready, set, go!
    banner::print_banner(&application_properties);

    let socket_address = make_address(&application_properties)?;

    print_system_log(
        Level::Info,
        format!("Starting Application on {}", socket_address).as_str(),
    );

    let listener = TcpListener::bind(socket_address).await?;

    let application_properties = Arc::new(application_properties);

    // 4. for graceful shutdown
    let service_avaliable = Arc::new(AtomicBool::new(true));
    let is_graceful_shutdown = application_properties.server.is_graceful_shutdown();
    let running_task_count = Arc::new(AtomicU64::new(0));

    if is_graceful_shutdown {
        graceful::handle_graceful_shutdown(
            &application_properties,
            Arc::clone(&service_avaliable),
            Arc::clone(&running_task_count),
        );
    }

    let keep_alive = application_properties.server.http1.keep_alive.to_owned();
    let http2_enabled = application_properties.server.http2.enabled.to_owned();

    #[cfg(feature = "tls")]
    let tls_acceptor = {
        print_system_log(Level::Info, "TLS Enabled");

        tls::new_tls_acceptor(&application_properties)?
    };

    // 5. Main Server Loop
    // Spawns a new async Task for each request.
    loop {
        let (mut tcp_stream, _) = listener.accept().await?;

        if is_graceful_shutdown {
            if !service_avaliable.load(std::sync::atomic::Ordering::Acquire) {
                print_system_log(Level::Info, "Service is not available");

                // reject new request
                use tokio::io::AsyncWriteExt;
                let _ = tcp_stream.shutdown();

                continue;
            }
        }

        // copy for each request
        let di_context = Arc::clone(&di_context);
        let application_properties = Arc::clone(&application_properties);
        let root_module = root_module.clone();

        // for Graceful Shutdown
        let running_task_count = Arc::clone(&running_task_count);

        #[cfg(feature = "tls")]
        let tls_acceptor = tls_acceptor.clone();

        // 6. create tokio task per HTTP request
        tokio::task::spawn(async move {
            let service = service_fn(move |request: Request<hyper::body::Incoming>| {
                let di_context = Arc::clone(&di_context);
                let application_properties = Arc::clone(&application_properties);

                let running_task_count = Arc::clone(&running_task_count);

                async move {
                    if let Some(timeout_duration) = application_properties.server.request_timeout {
                        let now = Instant::now();

                        let handle = tokio::time::timeout_at(
                            now + timeout_duration,
                            tokio::task::spawn(async move {
                                if is_graceful_shutdown {
                                    running_task_count
                                        .fetch_add(1, std::sync::atomic::Ordering::Release);
                                }

                                let result = process_request(
                                    application_properties,
                                    di_context,
                                    root_module,
                                    request,
                                )
                                .await;

                                if is_graceful_shutdown {
                                    running_task_count
                                        .fetch_sub(1, std::sync::atomic::Ordering::Release);
                                }

                                result
                            }),
                        );

                        match handle.await {
                            Ok(Ok(response)) => response,
                            Ok(Err(error)) => default_join_error_handler(error),
                            Err(error) => default_timeout_handler(error),
                        }
                    } else {
                        let _running_task_count = Arc::clone(&running_task_count);

                        let handle = tokio::spawn(async move {
                            let running_task_count = _running_task_count;

                            if is_graceful_shutdown {
                                running_task_count
                                    .fetch_add(1, std::sync::atomic::Ordering::Release);
                            }

                            let result = process_request(
                                application_properties,
                                di_context,
                                root_module,
                                request,
                            )
                            .await;

                            if is_graceful_shutdown {
                                running_task_count
                                    .fetch_sub(1, std::sync::atomic::Ordering::Release);
                            }

                            result
                        });

                        let result = handle.await;

                        let response = match result {
                            Ok(response) => response,
                            Err(error) => default_join_error_handler(error),
                        };

                        return response;
                    }
                }
            });

            #[cfg(feature = "tls")]
            let io = {
                let tls_stream = match tls_acceptor.accept(tcp_stream).await {
                    Ok(tls_stream) => tls_stream,
                    Err(err) => {
                        eprintln!("failed to perform tls handshake: {err:#}");
                        return;
                    }
                };

                TokioIo::new(tls_stream)
            };

            #[cfg(not(feature = "tls"))]
            let io = TokioIo::new(tcp_stream);

            if http2_enabled {
                let mut http_builder =
                    hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());

                http_builder.http2().enable_connect_protocol();

                if let Err(err) = http_builder
                    .serve_connection_with_upgrades(io, service)
                    .await
                {
                    println!("Error serving connection: {:?}", err);
                }
            } else {
                let mut http_builder = hyper::server::conn::http1::Builder::new();

                if keep_alive {
                    http_builder.keep_alive(keep_alive);
                }

                if let Err(err) = http_builder.serve_connection(io, service).await {
                    println!("Error serving connection: {:?}", err);
                }
            }
        });
    }
}

#[cfg(feature = "aws-lambda")]
pub async fn run_server_on_aws_lambda(
    application_properties: application_properties::ApplicationProperties,
    root_module: impl IModule + Clone + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    use bootings::aws_lambda::LambdaError;

    // 1. DI Context Initialize
    let mut di_context = di::DIContext::new();
    di_context.initialize(Box::new(root_module.clone()));
    let di_context = Arc::new(di_context);

    // 2. Prepare Swagger Serving, if enabled
    if let Some(swagger_context) = di_context.get::<SwaggerContext>() {
        swagger_context.initialize_from_module(root_module.clone());
    }

    // 3. ready, set, go!
    banner::print_banner(&application_properties);

    let application_properties = Arc::new(application_properties);

    loop {
        // 4. extract request context from AWS Lambda event
        let lambda_request_context = bootings::aws_lambda::get_request_context().await?;

        let di_context = Arc::clone(&di_context);
        let application_properties = Arc::clone(&application_properties);
        let root_module = root_module.clone();

        tokio::spawn(async move {
            let aws_request_id = lambda_request_context.aws_request_id.clone();

            let result = handle_event_on_aws_lambda(
                lambda_request_context,
                application_properties,
                di_context,
                root_module,
            )
            .await;

            if let Err(error) = result {
                bootings::aws_lambda::send_error_to_lambda(
                    aws_request_id.as_str(),
                    LambdaError {
                        error_message: error.to_string(),
                        error_type: "InternalServerError".to_string(),
                        stack_trace: Default::default(),
                    },
                )
                .await
                .unwrap();
            }
        });
    }
}

#[cfg(feature = "aws-lambda")]
pub async fn handle_event_on_aws_lambda(
    mut lambda_request_context: LambdaRequestEvent,
    application_properties: Arc<application_properties::ApplicationProperties>,
    di_context: Arc<di::DIContext>,
    root_module: impl IModule + Clone + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    use bootings::aws_lambda::LambdaReponse;

    if lambda_request_context.status_code == 204 {
        // Ignore the event if the status code is 204.
        // This is a way to keep the runtime alive when
        // there are no events pending to be processed.

        return Ok(());
    }

    let hyper_request_builder = hyper::Request::builder()
        .method(
            lambda_request_context
                .event_payload
                .request_context
                .http
                .method
                .as_str(),
        )
        .uri(lambda_request_context.event_payload.to_full_url());

    let body = std::mem::take(&mut lambda_request_context.event_payload.body);

    let mut hyper_request = hyper_request_builder.body(body.unwrap_or_default())?;

    *hyper_request.headers_mut() = lambda_request_context.event_payload.to_hyper_headermap();

    // 5. process request
    let mut response = process_request(
        application_properties,
        di_context,
        root_module,
        hyper_request,
    )
    .await?;

    // 6. convert response to AWS Lambda response format
    let status_code = response.status();
    let headermap = response.headers();

    let mut headers = HashMap::new();

    for (header_name, header_value) in headermap {
        let header_name = header_name.to_string();
        let header_value = header_value.to_str().unwrap_or("").to_string();

        headers.insert(header_name, header_value);
    }

    let response_body = match response.body_mut().collect().await {
        Ok(body) => {
            let body = body.to_bytes();
            let body = String::from_utf8(body.to_vec()).unwrap_or("".to_string());

            body
        }
        Err(error) => {
            return Err(anyhow::Error::from(error));
        }
    };

    // 7. send response to AWS Lambda
    bootings::aws_lambda::send_response_to_lambda(
        &lambda_request_context.aws_request_id,
        LambdaReponse {
            status_code: status_code.as_u16(),
            headers,
            body: response_body,
        },
    )
    .await?;

    Ok(())
}

fn make_address(
    application_properties: &application_properties::ApplicationProperties,
) -> anyhow::Result<SocketAddr> {
    use std::net::{IpAddr, SocketAddr};
    use std::str::FromStr;

    let port = application_properties.server.port;
    let host = application_properties.server.address.clone();

    let ip = IpAddr::from_str(host.as_str())?;

    let socket_addr = SocketAddr::new(ip, port);

    Ok(socket_addr)
}

fn default_404_handler() -> Result<Response<Full<Bytes>>, Infallible> {
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from("Not Found")))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(404) {
        *response.status_mut() = status;
    }

    return Ok::<Response<Full<Bytes>>, Infallible>(response);
}

fn default_timeout_handler(error: Elapsed) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from(format!(
            "Request Timeout: {}",
            error.to_string()
        ))))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(500) {
        *response.status_mut() = status;
    }

    return Ok::<Response<Full<Bytes>>, Infallible>(response);
}

fn default_join_error_handler(error: impl Error) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from(format!(
            "Internal Server Error: {:?}",
            error
        ))))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(500) {
        *response.status_mut() = status;
    }

    return Ok::<Response<Full<Bytes>>, Infallible>(response);
}

async fn process_request<T>(
    application_properties: Arc<application_properties::ApplicationProperties>,
    di_context: Arc<di::DIContext>,
    root_module: impl IModule + Clone + Copy + Send + Sync + 'static,
    request: Request<T>,
) -> Result<Response<Full<Bytes>>, Infallible>
where
    T: hyper::body::Body,
{
    // 1. Prepare URI matching
    let di_context = Arc::clone(&di_context);

    let uri = request.uri();
    let request_path = uri.path();
    let request_method = request.method();

    print_system_log(
        Level::Info,
        format!("[Request] {} {}", request_method, request_path).as_str(),
    );

    // 2. Find the one that matches the current request among the routes included in the hierarchical module structure.
    let found_route = route::find_route(Box::new(root_module), request_path, request_method);

    let found_route = match found_route {
        Some(route) => route,
        // TODO: 404 Handler Customization
        None => {
            return default_404_handler();
        }
    };

    // 3. Get the handler function for the matched route value,
    // prepare the request context, and pass it to the handler function.
    let (route, route_path, middlewares) = found_route;

    let handler = route.handler();

    let raw_querystring = uri.query().unwrap_or_default();

    // 3.1. Parse Query Parameters
    let query_parameters = parse::parse_query_parameter(raw_querystring);

    // 3.2. Parse Headers
    let mut headers = HashMap::new();
    for (header_name, header_value) in request.headers() {
        let header_name = header_name.to_string();
        let header_value = header_value.to_str().unwrap_or("").to_string();

        headers.insert(header_name, header_value);
    }
    preprocess_headers(&mut headers);

    // 3.3. Parse Path Parameters
    let path_parameters = parse::parse_path_parameter(route_path, request_path);

    let request_method = request_method.to_owned();
    let request_path = request_path.to_owned();

    let request_body = match request.collect().await {
        Ok(body) => {
            let body = body.to_bytes();
            let body = String::from_utf8(body.to_vec()).unwrap_or("".to_string());

            body
        }
        Err(_) => {
            return Ok::<Response<Full<Bytes>>, Infallible>(Response::new(Full::new(Bytes::from(
                format!("Error reading request body"),
            ))));
        }
    };

    // 3.4. Call the handler function
    let response = std::panic::catch_unwind(move || {
        let mut request = crate::Request {
            method: request_method,
            path: request_path,
            body: request_body,
            query_parameters,
            headers,
            path_parameters,
            cookies: HashMap::new(),
            di_context: Arc::clone(&di_context),
        };

        request.parse_cookies_from_headers();

        let mut response = crate::Response::new();

        // 3.5. middleware chain processing
        for middleware in middlewares {
            let middleware_result =
                middleware(request, response.clone(), move |request, response| {
                    let next = Some(Box::new((request, response)));

                    let mut response = crate::Response::new();
                    response.next = next;

                    response
                });

            match middleware_result.next {
                Some(next) => {
                    let (next_request, next_response) = *next;

                    request = next_request;
                    response = next_response;
                }
                None => {
                    return middleware_result;
                }
            }
        }

        handler.handle(request, response)
    });

    // 4. Unhandled Error Handling
    let response = match response {
        Ok(response) => response,
        Err(_err) => crate::Response::new()
            .status(500)
            .text("Internal Server Error".to_string()),
    };

    // 5. Post-Processing Response
    // ex) Compression, etc.
    let response = post_process_response(application_properties, response);

    let status = response.status.clone();
    let headers = response.headers.clone();

    // 6. Convert to hyper::Response, and return
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from(response.body)))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(status) {
        *response.status_mut() = status;
    }

    for (key, values) in headers.iter() {
        for value in values.iter() {
            if let Ok(value) = value.parse() {
                response.headers_mut().append(key, value);
            }
        }
    }

    return Ok::<Response<Full<Bytes>>, Infallible>(response);
}

fn post_process_response(
    application_properties: Arc<application_properties::ApplicationProperties>,
    mut response: crate::Response,
) -> crate::Response {
    if !application_properties.server.compression.enabled {
        return response;
    }

    let content_types = response
        .headers
        .get(&crate::HeaderName::from_static(header::CONTENT_TYPE));

    let content_types = match content_types {
        Some(content_types) => content_types,
        None => return response,
    };

    let mut is_compression_content_type = false;
    for content_type in content_types {
        if application_properties
            .server
            .compression
            .mime_types
            .contains(content_type)
        {
            is_compression_content_type = true;
            break;
        }
    }

    if !is_compression_content_type {
        return response;
    }

    if response.body.len() < application_properties.server.compression.min_response_size {
        return response;
    }

    match application_properties.server.compression.algorithm {
        CompressionAlgorithm::Gzip => {
            // compression
            let compressed_bytes = compression::compress_with_gzip(&response.body);

            let compressed_bytes = match compressed_bytes {
                Ok(compressed_bytes) => compressed_bytes,
                Err(err) => {
                    eprintln!("Error compressing response body: {:?}", err);
                    return response;
                }
            };

            response.body = compressed_bytes;

            // add header for compression
            response.headers.insert(
                crate::HeaderName::from_static(header::CONTENT_ENCODING),
                vec![application_properties
                    .server
                    .compression
                    .algorithm
                    .to_string()],
            );
        }
        CompressionAlgorithm::Deflate => {
            // compression
            let compressed_bytes = compression::compress_with_deflate(&response.body);

            let compressed_bytes = match compressed_bytes {
                Ok(compressed_bytes) => compressed_bytes,
                Err(err) => {
                    eprintln!("Error compressing response body: {:?}", err);
                    return response;
                }
            };

            response.body = compressed_bytes;

            // add header for compression
            response.headers.insert(
                crate::HeaderName::from_static(header::CONTENT_ENCODING),
                vec![application_properties
                    .server
                    .compression
                    .algorithm
                    .to_string()],
            );
        }
        _ => {}
    }

    response
}
