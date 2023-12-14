use std::collections::HashMap;

use http_body_util::Full;
use hyper::{body::Bytes, header::HeaderName};

#[derive(Debug, Clone, Default)]
pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<HeaderName, String>,
}

impl Response {
    /// Create a new response with status code 200, empty body and empty headers.
    /// ```
    /// let response = rupring::Response::new();
    /// // ...
    /// ```
    pub fn new() -> Self {
        Self {
            status: 200,
            body: "".to_string(),
            headers: Default::default(),
        }
    }
}

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl From<Response> for hyper::Response<Full<Bytes>> {
    fn from(response: Response) -> Self {
        let mut builder = hyper::Response::builder();

        builder = builder.status(response.status);

        for (header_name, header_value) in response.headers {
            builder = builder.header(header_name.clone(), header_value);
        }

        let response = builder.body(Full::new(Bytes::from(response.body))).unwrap();

        return response;
    }
}
