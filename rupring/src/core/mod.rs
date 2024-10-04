mod banner;
pub mod boot;
mod graceful;
mod parse;

use crate::application_properties;
use crate::di;
use crate::header;
pub(crate) mod route;

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
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
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut di_context = di::DIContext::new();
    di_context.initialize(Box::new(root_module.clone()));
    let di_context = Arc::new(di_context);

    if let Some(swagger_context) = di_context.get::<SwaggerContext>() {
        swagger_context.initialize_from_module(root_module.clone());
    }

    banner::print_banner();

    let socket_address = make_address(&application_properties)?;

    print_system_log(
        Level::Info,
        format!("Starting Application on {}", socket_address).as_str(),
    );

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(socket_address).await?;

    let application_properties = Arc::new(application_properties);

    // for graceful shutdown
    let signal_flags = graceful::SignalFlags::new();
    let service_avaliable = Arc::new(AtomicBool::new(true));

    let is_graceful_shutdown = application_properties.server.is_graceful_shutdown();
    let shutdown_timeout_duration = application_properties.server.shutdown_timeout_duration();
    let running_task_count = Arc::new(AtomicU64::new(0));

    if is_graceful_shutdown {
        if let Err(error) = signal_flags.register_hooks() {
            print_system_log(
                Level::Error,
                format!("Error registering signal hooks: {:?}", error).as_str(),
            );
        } else {
            print_system_log(Level::Info, "Graceful shutdown enabled");

            let service_avaliable = Arc::clone(&service_avaliable);
            let running_task_count = Arc::clone(&running_task_count);
            tokio::spawn(async move {
                let sigterm = Arc::clone(&signal_flags.sigterm);
                let sigint = Arc::clone(&signal_flags.sigint);

                loop {
                    if sigterm.load(std::sync::atomic::Ordering::Relaxed) {
                        print_system_log(
                            Level::Info,
                            "SIGTERM received. Try to shutdown gracefully...",
                        );
                        service_avaliable.store(false, std::sync::atomic::Ordering::Release);
                        break;
                    }

                    if sigint.load(std::sync::atomic::Ordering::Relaxed) {
                        print_system_log(
                            Level::Info,
                            "SIGINT received. Try to shutdown gracefully...",
                        );
                        service_avaliable.store(false, std::sync::atomic::Ordering::Release);
                        break;
                    }

                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }

                let shutdown_request_time = std::time::Instant::now();

                loop {
                    if running_task_count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                        print_system_log(Level::Info, "All tasks are done. Shutting down...");
                        std::process::exit(0);
                    }

                    // timeout 지나면 강제로 종료
                    let now = std::time::Instant::now();
                    if now.duration_since(shutdown_request_time) >= shutdown_timeout_duration {
                        print_system_log(
                            Level::Info,
                            "Shutdown timeout reached. Forcing shutdown...",
                        );
                        std::process::exit(0);
                    }

                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            });
        }
    }

    // We start a loop to continuously accept incoming connections
    loop {
        let (mut stream, _) = listener.accept().await?;

        if is_graceful_shutdown {
            if !service_avaliable.load(std::sync::atomic::Ordering::Acquire) {
                print_system_log(Level::Info, "Service is not available");

                // reject new request
                use tokio::io::AsyncWriteExt;
                let _ = stream.shutdown();

                continue;
            }
        }

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        let di_context = Arc::clone(&di_context);

        let application_properties = Arc::clone(&application_properties);

        let root_module = root_module.clone();

        let running_task_count = Arc::clone(&running_task_count);

        // create tokio task per HTTP request
        tokio::task::spawn(async move {
            if is_graceful_shutdown {
                running_task_count.fetch_add(1, std::sync::atomic::Ordering::Release);
            }

            if let Err(err) = http1::Builder::new()
                .keep_alive(true)
                // `service_fn` converts our function in a `Service`
                .serve_connection(
                    io,
                    service_fn(|req: Request<hyper::body::Incoming>| {
                        let di_context = Arc::clone(&di_context);
                        let application_properties = Arc::clone(&application_properties);

                        async move {
                            process_request(application_properties, di_context, root_module, req)
                                .await
                        }
                    }),
                )
                .await
            {
                println!("Error serving connection: {:?}", err);
            }

            if is_graceful_shutdown {
                running_task_count.fetch_sub(1, std::sync::atomic::Ordering::Release);
            }
        });
    }
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

async fn process_request(
    application_properties: Arc<application_properties::ApplicationProperties>,
    di_context: Arc<di::DIContext>,
    root_module: impl IModule + Clone + Copy + Send + Sync + 'static,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let di_context = Arc::clone(&di_context);

    let uri = req.uri();
    let request_path = uri.path().to_string();
    let request_method = req.method().to_owned();

    print_system_log(
        Level::Info,
        format!("[Request] {} {}", request_method, request_path).as_str(),
    );

    let found_route = route::find_route(
        Box::new(root_module),
        request_path.clone(),
        request_method.clone(),
    );

    let found_route = match found_route {
        Some(route) => route,
        None => {
            return Ok::<Response<Full<Bytes>>, Infallible>(Response::new(Full::new(Bytes::from(
                "Not Found".to_string(),
            ))));
        }
    };

    let (route, route_path, middlewares) = found_route;

    let handler = route.handler();

    let raw_querystring = uri.query().unwrap_or("");
    let query_parameters = parse::parse_query_parameter(raw_querystring);

    let mut headers = HashMap::new();
    for (header_name, header_value) in req.headers() {
        let header_name = header_name.to_string();
        let header_value = header_value.to_str().unwrap_or("").to_string();

        headers.insert(header_name, header_value);
    }

    preprocess_headers(&mut headers);

    let path_parameters = parse::parse_path_parameter(route_path, request_path.clone());

    let request_body = match req.collect().await {
        Ok(body) => {
            let body = body.to_bytes();
            let body = String::from_utf8(body.to_vec()).unwrap_or("".to_string());

            body
        }
        Err(err) => {
            return Ok::<Response<Full<Bytes>>, Infallible>(Response::new(Full::new(Bytes::from(
                format!("Error reading request body: {:?}", err),
            ))));
        }
    };

    let response = std::panic::catch_unwind(move || {
        let mut request = crate::Request {
            method: request_method,
            path: request_path,
            body: request_body,
            query_parameters,
            headers,
            path_parameters,
            di_context: Arc::clone(&di_context),
        };

        let mut response = crate::Response::new();

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

    let response = match response {
        Ok(response) => response,
        Err(_err) => crate::Response::new()
            .status(500)
            .text("Internal Server Error".to_string()),
    };

    let response = post_process_response(application_properties, response);

    let status = response.status.clone();
    let headers = response.headers.clone();

    // ---- 최종 응답 처리 ----
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from(response.body)))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(status) {
        *response.status_mut() = status;
    }

    for (key, value) in headers.iter() {
        if let Ok(value) = value.parse() {
            response.headers_mut().insert(key, value);
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

    let content_type = response
        .headers
        .get(&crate::HeaderName::from_static(header::CONTENT_TYPE));

    let content_type = match content_type {
        Some(content_type) => content_type,
        None => return response,
    };

    if !application_properties
        .server
        .compression
        .mime_types
        .contains(content_type)
    {
        return response;
    }

    if response.body.len() < application_properties.server.compression.min_response_size {
        return response;
    }

    match application_properties.server.compression.algorithm.as_str() {
        "gzip" => {
            // compression
            let compressed_bytes = compress_with_gzip(&response.body);

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
                application_properties
                    .server
                    .compression
                    .algorithm
                    .to_string(),
            );
        }
        "deflate" => {
            // compression
            let compressed_bytes = compress_with_deflate(&response.body);

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
                application_properties
                    .server
                    .compression
                    .algorithm
                    .to_string(),
            );
        }
        _ => {}
    }

    response
}

fn compress_with_gzip(body: &[u8]) -> anyhow::Result<Vec<u8>> {
    use std::io::Write;

    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(body)?;
    let compressed = encoder.finish()?;

    Ok(compressed)
}

fn compress_with_deflate(body: &[u8]) -> anyhow::Result<Vec<u8>> {
    use std::io::Write;

    let mut encoder =
        flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(body)?;
    let compressed = encoder.finish()?;

    Ok(compressed)
}
