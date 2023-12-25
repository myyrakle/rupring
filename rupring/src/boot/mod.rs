mod banner;
pub mod di;
mod parse;
mod route;

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
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
use crate::IModule;

pub async fn run_server(
    socker_addr: SocketAddr,
    root_module: impl IModule + Clone + Copy + Send + Sync + 'static,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut di_context = di::DIContext::new();
    di_context.initialize(Box::new(root_module.clone()));
    let di_context = Arc::new(di_context);

    banner::print_banner();

    print_system_log(
        Level::Info,
        format!("Starting Application on {}", socker_addr).as_str(),
    );

    let addr = socker_addr;

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        let di_context = Arc::clone(&di_context);

        let root_module = root_module.clone();
        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(
                    io,
                    service_fn(|req: Request<hyper::body::Incoming>| {
                        let di_context = Arc::clone(&di_context);

                        async move {
                            let di_context = Arc::clone(&di_context);

                            let uri = req.uri();
                            let request_path = uri.path().to_string();
                            let request_method = req.method().to_owned();

                            print_system_log(
                                Level::Info,
                                format!("[Request] {} {}", request_method, request_path).as_str(),
                            );

                            let route = route::find_route(
                                Box::new(root_module),
                                request_path.clone(),
                                request_method.clone(),
                            );

                            match route {
                                Some((route, route_path, middlewares)) => {
                                    let handler = route.handler();

                                    let raw_querystring = uri.query().unwrap_or("");
                                    let query_parameters =
                                        parse::parse_query_parameter(raw_querystring);

                                    let mut headers = HashMap::new();
                                    for (header_name, header_value) in req.headers() {
                                        let header_name = header_name.to_string();
                                        let header_value =
                                            header_value.to_str().unwrap_or("").to_string();

                                        headers.insert(header_name, header_value);
                                    }

                                    preprocess_headers(&mut headers);

                                    let path_parameters = parse::parse_path_parameter(
                                        route_path,
                                        request_path.clone(),
                                    );

                                    let request_body = match req.collect().await {
                                        Ok(body) => {
                                            let body = body.to_bytes();
                                            let body = String::from_utf8(body.to_vec())
                                                .unwrap_or("".to_string());

                                            body
                                        }
                                        Err(err) => {
                                            return Ok::<Response<Full<Bytes>>, Infallible>(
                                                Response::new(Full::new(Bytes::from(format!(
                                                    "Error reading request body: {:?}",
                                                    err
                                                )))),
                                            );
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
                                            let middleware_result = middleware(
                                                request,
                                                response.clone(),
                                                move |request, response| {
                                                    let next = Some(Box::new((request, response)));

                                                    let mut response = crate::Response::new();
                                                    response.next = next;

                                                    response
                                                },
                                            );

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

                                    let headers = response.headers.clone();
                                    let status = response.status.clone();

                                    let mut response: hyper::Response<Full<Bytes>> =
                                        response.into();

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
                                None => {
                                    return Ok::<Response<Full<Bytes>>, Infallible>(Response::new(
                                        Full::new(Bytes::from("Not Found".to_string())),
                                    ));
                                }
                            }
                        }
                    }),
                )
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
