use bytes::Bytes;
use http_body_util::Full;
use hyper::{Response, StatusCode};
use std::convert::Infallible;
use std::error::Error;
use tokio::time::error::Elapsed;

pub(crate) fn default_payload_too_large_handler() -> Result<Response<Full<Bytes>>, Infallible> {
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from("Payload Too Large")))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(413) {
        *response.status_mut() = status;
    }

    Ok::<Response<Full<Bytes>>, Infallible>(response)
}

pub(crate) fn default_404_handler() -> Result<Response<Full<Bytes>>, Infallible> {
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from("Not Found")))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(404) {
        *response.status_mut() = status;
    }

    Ok::<Response<Full<Bytes>>, Infallible>(response)
}

pub(crate) fn default_uri_too_long_handler() -> Result<Response<Full<Bytes>>, Infallible> {
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from("URI Too Long")))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(414) {
        *response.status_mut() = status;
    }

    Ok::<Response<Full<Bytes>>, Infallible>(response)
}

pub(crate) fn default_header_size_too_big() -> Result<Response<Full<Bytes>>, Infallible> {
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from("Header Size Too Big")))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(400) {
        *response.status_mut() = status;
    }

    Ok::<Response<Full<Bytes>>, Infallible>(response)
}

pub(crate) fn default_header_fields_to_large() -> Result<Response<Full<Bytes>>, Infallible> {
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from("Request Header Fields Too Large")))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(431) {
        *response.status_mut() = status;
    }

    Ok::<Response<Full<Bytes>>, Infallible>(response)
}

pub(crate) fn default_timeout_handler(error: Elapsed) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from(format!("Request Timeout: {error}",))))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(500) {
        *response.status_mut() = status;
    }

    Ok::<Response<Full<Bytes>>, Infallible>(response)
}

pub(crate) fn default_join_error_handler(
    error: impl Error,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut response: hyper::Response<Full<Bytes>> = Response::builder()
        .body(Full::new(Bytes::from(format!(
            "Internal Server Error: {:?}",
            error
        ))))
        .unwrap();

    if let Ok(status) = StatusCode::from_u16(500) {
        *response.status_mut() = status;
    }

    Ok::<Response<Full<Bytes>>, Infallible>(response)
}
