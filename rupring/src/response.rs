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

    /// Set it to receive the value of a serializable object and return a json value.
    /// ```
    /// #[derive(serde::Serialize)]
    /// struct User {
    ///    name: String,
    /// }
    ///
    /// let response = rupring::Response::new().json(User {
    ///    name: "John".to_string(),
    /// });
    /// assert_eq!(response.body, r#"{"name":"John"}"#);
    /// // ...
    /// ```
    pub fn json(mut self, body: impl serde::Serialize) -> Self {
        self.headers.insert(
            HeaderName::from_static("content-type"),
            "application/json".to_string(),
        );

        self.body = match serde_json::to_string(&body) {
            Ok(body) => body,
            Err(err) => {
                self.status = 500;
                format!("Error serializing response body: {:?}", err)
            }
        };

        return self;
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
