mod banner;
pub mod boot;
pub(crate) mod bootings;
mod compression;
pub(crate) mod cookie;
mod error_handler;
mod graceful;
mod parse;

#[cfg(feature = "aws-lambda")]
use bootings::aws_lambda::LambdaRequestEvent;

#[cfg(feature = "tls")]
use bootings::tls;
use error_handler::default_404_handler;
use error_handler::default_header_fields_to_large;
use error_handler::default_header_size_too_big;
use error_handler::default_join_error_handler;
use error_handler::default_payload_too_large_handler;
use error_handler::default_timeout_handler;
use error_handler::default_uri_too_long_handler;
use http_body_util::{BodyExt, Full, Limited};
use tokio::time::Instant;

use crate::application_properties;
use crate::application_properties::ApplicationProperties;
use crate::application_properties::CompressionAlgorithm;
use crate::di;
use crate::header;
use crate::http::multipart;
use crate::request::Metadata;
pub(crate) mod route;

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::IpAddr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use std::vec;

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

struct RequestAdditionalData {
    pub ip: IpAddr,
    pub(crate) request_body_on_aws_lambda: Option<String>, // in AWS Lambda mode
}

pub async fn run_server(
    application_properties: application_properties::ApplicationProperties,
    root_module: impl IModule + Clone + Send + Sync + 'static,
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

    let socket_address = application_properties.server.make_address()?;

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

    #[cfg(feature = "tls")]
    let tls_acceptor = {
        print_system_log(Level::Info, "TLS Enabled");

        tls::new_tls_acceptor(&application_properties)?
    };

    // 5. Main Server Loop
    // Spawns a new async Task for each request.
    loop {
        let (mut tcp_stream, _) = listener.accept().await?;

        let ip = tcp_stream.peer_addr()?.ip();

        if is_graceful_shutdown && !service_avaliable.load(std::sync::atomic::Ordering::Acquire) {
            print_system_log(Level::Info, "Service is not available");

            // reject new request
            use tokio::io::AsyncWriteExt;
            let shutdown_task = tcp_stream.shutdown();

            std::mem::drop(shutdown_task);

            continue;
        }

        // copy for each request
        let di_context = Arc::clone(&di_context);
        let application_properties = Arc::clone(&application_properties);
        let root_module = root_module.clone();

        let max_number_of_headers = application_properties
            .server
            .request
            .header
            .max_number_of_headers;

        // for Graceful Shutdown
        let running_task_count = Arc::clone(&running_task_count);

        #[cfg(feature = "tls")]
        let tls_acceptor = tls_acceptor.clone();

        // 6. create tokio task per HTTP request
        tokio::task::spawn(async move {
            let service = service_fn(move |request: Request<hyper::body::Incoming>| {
                handle_http_connection(
                    Arc::clone(&application_properties),
                    Arc::clone(&di_context),
                    root_module.clone(),
                    request,
                    ip,
                    is_graceful_shutdown,
                    Arc::clone(&running_task_count),
                )
            });

            #[cfg(feature = "tls")]
            let io = {
                let tls_stream = match tls_acceptor.accept(tcp_stream).await {
                    Ok(tls_stream) => tls_stream,
                    Err(err) => {
                        log::error!("failed to perform tls handshake: {err:#}");
                        return;
                    }
                };

                TokioIo::new(tls_stream)
            };

            #[cfg(not(feature = "tls"))]
            let io = TokioIo::new(tcp_stream);

            {
                let mut http_builder = hyper::server::conn::http1::Builder::new();

                if let Some(max_number_of_headers) = max_number_of_headers {
                    http_builder.max_headers(max_number_of_headers);
                }

                if keep_alive {
                    http_builder.keep_alive(keep_alive);
                }

                if let Err(err) = http_builder.serve_connection(io, service).await {
                    if err.is_parse_too_large() {
                        return;
                    }
                    log::debug!("Error serving connection: {:?}", err);
                }
            }
        });
    }
}

async fn handle_http_connection(
    application_properties: Arc<ApplicationProperties>,
    di_context: Arc<di::DIContext>,
    root_module: impl IModule + Clone + Send + Sync + 'static,
    request: Request<hyper::body::Incoming>,
    ip: IpAddr,
    is_graceful_shutdown: bool,
    running_task_count: Arc<AtomicU64>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let request_additional_data = RequestAdditionalData {
        ip,
        request_body_on_aws_lambda: None,
    };

    let di_context = Arc::clone(&di_context);
    let application_properties = Arc::clone(&application_properties);

    let running_task_count = Arc::clone(&running_task_count);
    let root_module = root_module.clone();

    if let Some(max_length) = application_properties.server.request.uri.max_length {
        let incoming_url_length = request
            .uri()
            .path_and_query()
            .map(|e| e.as_str().len())
            .unwrap_or_default();

        if incoming_url_length > max_length {
            return default_uri_too_long_handler();
        }
    }

    if let Some(timeout_duration) = application_properties.server.request_timeout {
        let now = Instant::now();

        let handle = tokio::time::timeout_at(
            now + timeout_duration,
            tokio::task::spawn(async move {
                if is_graceful_shutdown {
                    running_task_count.fetch_add(1, std::sync::atomic::Ordering::Release);
                }

                let result = execute_request_pipeline(
                    application_properties,
                    di_context,
                    root_module,
                    request,
                    request_additional_data,
                    ProcessRequestOption {
                        boot_mode: BootMode::Normal,
                    },
                )
                .await;

                if is_graceful_shutdown {
                    running_task_count.fetch_sub(1, std::sync::atomic::Ordering::Release);
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
                running_task_count.fetch_add(1, std::sync::atomic::Ordering::Release);
            }

            let result = execute_request_pipeline(
                application_properties,
                di_context,
                root_module,
                request,
                request_additional_data,
                ProcessRequestOption {
                    boot_mode: BootMode::Normal,
                },
            )
            .await;

            if is_graceful_shutdown {
                running_task_count.fetch_sub(1, std::sync::atomic::Ordering::Release);
            }

            result
        });

        let result = handle.await;

        match result {
            Ok(response) => response,
            Err(error) => default_join_error_handler(error),
        }
    }
}

#[cfg(feature = "aws-lambda")]
pub async fn run_server_on_aws_lambda(
    application_properties: application_properties::ApplicationProperties,
    root_module: impl IModule + Clone + Send + Sync + 'static,
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
    root_module: impl IModule + Clone + Send + Sync + 'static,
) -> anyhow::Result<()> {
    use bootings::aws_lambda::LambdaReponse;
    use hyper::{body::Incoming, Version};

    if lambda_request_context.status_code == 204 {
        // Ignore the event if the status code is 204.
        // This is a way to keep the runtime alive when
        // there are no events pending to be processed.

        return Ok(());
    }

    let version = match lambda_request_context
        .event_payload
        .request_context
        .http
        .protocol
        .as_str()
    {
        "HTTP/1.1" => Version::HTTP_11,
        "HTTP/2" => Version::HTTP_2,
        _ => Version::HTTP_11,
    };

    let hyper_request_builder = hyper::Request::builder()
        .version(version)
        .method(
            lambda_request_context
                .event_payload
                .request_context
                .http
                .method
                .as_str(),
        )
        .uri(lambda_request_context.event_payload.to_full_url());

    let body = std::mem::take(&mut lambda_request_context.event_payload.body).unwrap_or_default();

    let mut hyper_request = hyper_request_builder.body(Incoming::empty())?;

    *hyper_request.headers_mut() = lambda_request_context.event_payload.to_hyper_headermap();

    let request_additional_data = RequestAdditionalData {
        ip: lambda_request_context
            .event_payload
            .request_context
            .http
            .source_ip
            .parse()
            .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0))),
        request_body_on_aws_lambda: Some(body),
    };

    // 5. process request
    let mut response = execute_request_pipeline(
        application_properties,
        di_context,
        root_module,
        hyper_request,
        request_additional_data,
        ProcessRequestOption {
            boot_mode: BootMode::AWSLambda,
        },
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

            String::from_utf8(body.to_vec()).unwrap_or("".to_string())
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

enum BootMode {
    Normal,
    AWSLambda,
}

struct ProcessRequestOption {
    pub boot_mode: BootMode,
}

impl Default for ProcessRequestOption {
    fn default() -> Self {
        ProcessRequestOption {
            boot_mode: BootMode::Normal,
        }
    }
}

async fn execute_request_pipeline(
    application_properties: Arc<application_properties::ApplicationProperties>,
    di_context: Arc<di::DIContext>,
    root_module: impl IModule + Clone + Send + Sync + 'static,
    request: Request<hyper::body::Incoming>,
    request_additional_data: RequestAdditionalData,
    option: ProcessRequestOption,
) -> Result<Response<Full<Bytes>>, Infallible> {
    // 1. Prepare URI matching
    let di_context = Arc::clone(&di_context);

    let uri = request.uri();
    let request_path = uri.path();
    let request_method = request.method();
    let mut request_metadata = Metadata {
        ip: request_additional_data.ip,
        protocol: request.version().into(),
        ..Default::default()
    };

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
    let mut cookies = HashMap::new();

    // 3.1. Parse Query Parameters
    let query_parameters = parse::parse_query_parameter(raw_querystring);

    let mut multipart_boundary = None;

    // 3.2. Parse Headers
    let mut headers = HashMap::new();
    for (header_name, header_value) in request.headers() {
        let header_name = header_name.to_string();
        let header_value = header_value.to_str().unwrap_or("").to_string();

        request_metadata.header_size += header_name.len() + header_value.len();
        request_metadata.number_of_headers += 1;

        if let Some(header_max_length) = application_properties.server.request.header.max_length {
            if request_metadata.header_size > header_max_length {
                return default_header_size_too_big();
            }
        }

        if let Some(max_number_of_headers) = application_properties
            .server
            .request
            .header
            .max_number_of_headers
        {
            if request_metadata.number_of_headers > max_number_of_headers {
                return default_header_fields_to_large();
            }
        }

        if application_properties.server.multipart.auto_parsing_enabled
            && header_name == header::CONTENT_TYPE
            && header_value.starts_with("multipart/form-data")
        {
            multipart_boundary = multipart::parse_multipart_boundary(&header_value)
        }

        if application_properties.server.cookie.auto_parsing_enabled
            && header_name == header::COOKIE
        {
            cookies = cookie::parse_cookie_header(&header_value);
        }

        headers.insert(header_name, header_value);
    }
    preprocess_headers(&mut headers);

    // 3.3. Parse Path Parameters
    let path_parameters = parse::parse_path_parameter(route_path, request_path);

    let request_method = request_method.to_owned();
    let request_path = request_path.to_owned();

    let mut request_body = "".to_string();
    #[allow(unused_assignments)]
    let mut raw_request_body = vec![];
    let mut files = vec![];

    // request body limit (default: 2MB)
    let body_limit = application_properties.server.request.body.max_length;

    match option.boot_mode {
        BootMode::AWSLambda => {
            if let Some(request_body_on_aws_lambda) =
                request_additional_data.request_body_on_aws_lambda
            {
                if request_body_on_aws_lambda.len()
                    > application_properties.server.request.body.max_length
                {
                    return default_payload_too_large_handler();
                }

                request_body = request_body_on_aws_lambda;
            }

            // TODO: 멀티파트 파싱 로직 추가
        }
        BootMode::Normal => {
            let limited_request_body_stream = Limited::new(request, body_limit);

            match limited_request_body_stream.collect().await {
                Ok(body) => {
                    raw_request_body = body.to_bytes().to_vec();

                    if application_properties.server.multipart.auto_parsing_enabled {
                        if let Some(boundary) = multipart_boundary {
                            files = multipart::parse_multipart(&raw_request_body, &boundary)
                                .unwrap_or_default();
                        } else {
                            request_body = core::str::from_utf8(&raw_request_body)
                                .unwrap_or("")
                                .to_string();
                        }
                    }
                }
                Err(_) => {
                    return Ok::<Response<Full<Bytes>>, Infallible>(Response::new(Full::new(
                        Bytes::from("Error reading request body"),
                    )));
                }
            }
        }
    }

    // 3.4. Call the handler function
    let response = std::panic::catch_unwind(move || {
        let mut request = crate::Request {
            method: request_method,
            path: request_path,
            body: request_body,
            raw_body: raw_request_body,
            query_parameters,
            headers,
            path_parameters,
            cookies,
            files,
            metadata: request_metadata,
            di_context: Arc::clone(&di_context),
        };

        request.parse_cookies();

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

    let status = response.status;
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

    Ok::<Response<Full<Bytes>>, Infallible>(response)
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
