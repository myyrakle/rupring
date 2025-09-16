use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use std::convert::Infallible;
use std::error::Error;
use tokio::time::error::Elapsed;

use crate::core::ResponseBytesBody;

pub(crate) fn default_404_handler() -> Result<hyper::Response<ResponseBytesBody>, Infallible> {
    let mut response: hyper::Response<ResponseBytesBody> = hyper::Response::builder()
        .body(BodyExt::boxed(BoxBody::new("Not Found".to_string())))
        .unwrap();

    if let Ok(status) = hyper::StatusCode::from_u16(404) {
        *response.status_mut() = status;
    }

    Ok::<hyper::Response<ResponseBytesBody>, Infallible>(response)
}

pub(crate) fn default_payload_too_large_handler(
) -> Result<hyper::Response<ResponseBytesBody>, Infallible> {
    let mut response: hyper::Response<ResponseBytesBody> = hyper::Response::builder()
        .body(BodyExt::boxed(BoxBody::new(
            "Payload Too Large".to_string(),
        )))
        .unwrap();

    if let Ok(status) = hyper::StatusCode::from_u16(413) {
        *response.status_mut() = status;
    }

    Ok(response)
}

pub(crate) fn default_uri_too_long_handler(
) -> Result<hyper::Response<ResponseBytesBody>, Infallible> {
    let mut response: hyper::Response<ResponseBytesBody> = hyper::Response::builder()
        .body(BodyExt::boxed(BoxBody::new("URI Too Long".to_string())))
        .unwrap();

    if let Ok(status) = hyper::StatusCode::from_u16(414) {
        *response.status_mut() = status;
    }

    Ok(response)
}

pub(crate) fn default_header_size_too_big() -> Result<hyper::Response<ResponseBytesBody>, Infallible>
{
    let mut response: hyper::Response<ResponseBytesBody> = hyper::Response::builder()
        .body(BodyExt::boxed(BoxBody::new(
            "Header Size Too Big".to_string(),
        )))
        .unwrap();

    if let Ok(status) = hyper::StatusCode::from_u16(400) {
        *response.status_mut() = status;
    }

    Ok(response)
}

pub(crate) fn default_header_fields_to_large(
) -> Result<hyper::Response<ResponseBytesBody>, Infallible> {
    let mut response: hyper::Response<ResponseBytesBody> = hyper::Response::builder()
        .body(BodyExt::boxed(BoxBody::new(
            "Request Header Fields Too Large".to_string(),
        )))
        .unwrap();

    if let Ok(status) = hyper::StatusCode::from_u16(431) {
        *response.status_mut() = status;
    }

    Ok(response)
}

pub(crate) fn default_timeout_handler(
    error: Elapsed,
) -> Result<hyper::Response<ResponseBytesBody>, Infallible> {
    let mut response: hyper::Response<ResponseBytesBody> = hyper::Response::builder()
        .body(BodyExt::boxed(BoxBody::new(format!(
            "Request Timeout: {error}",
        ))))
        .unwrap();

    if let Ok(status) = hyper::StatusCode::from_u16(500) {
        *response.status_mut() = status;
    }

    Ok::<hyper::Response<ResponseBytesBody>, Infallible>(response)
}

pub(crate) fn default_join_error_handler(
    error: impl Error,
) -> Result<hyper::Response<ResponseBytesBody>, Infallible> {
    let mut response: hyper::Response<ResponseBytesBody> = hyper::Response::builder()
        .body(BodyExt::boxed(BoxBody::new(format!(
            "Internal Server Error: {error:?}",
        ))))
        .unwrap();

    if let Ok(status) = hyper::StatusCode::from_u16(500) {
        *response.status_mut() = status;
    }

    Ok::<hyper::Response<ResponseBytesBody>, Infallible>(response)
}
