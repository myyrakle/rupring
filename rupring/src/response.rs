use std::{collections::HashMap, panic::UnwindSafe};

use crate::{HeaderName, Request};
use http_body_util::Full;
use hyper::body::Bytes;

#[derive(Debug, Clone, Default)]
pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<HeaderName, String>,
    pub(crate) next: Option<Box<(Request, Response)>>,
}

impl UnwindSafe for Response {}

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
            next: None,
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
            crate::HeaderName::from_static("content-type"),
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

    /// Set to return a text value.
    /// ```
    /// let response = rupring::Response::new().text("Hello World".to_string());
    /// assert_eq!(response.body, "Hello World".to_string());
    pub fn text(mut self, body: impl ToString) -> Self {
        self.headers.insert(
            crate::HeaderName::from_static("content-type"),
            "text/plain".to_string(),
        );

        self.body = body.to_string();

        return self;
    }

    /// Set status code.
    /// ```
    /// let response = rupring::Response::new().status(404);
    /// assert_eq!(response.status, 404);
    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        return self;
    }

    /// Set a header.
    /// ```
    /// use rupring::HeaderName;
    /// let response = rupring::Response::new().header("content-type", "application/json".to_string());
    /// assert_eq!(response.headers.get(&HeaderName::from_static("content-type")).unwrap(), &"application/json".to_string());
    pub fn header(mut self, name: &'static str, value: String) -> Self {
        self.headers.insert(HeaderName::from_static(name), value);
        return self;
    }

    /// overwrite headers.
    /// ```
    /// use rupring::HeaderName;
    /// use std::collections::HashMap;
    /// let mut headers = HashMap::new();
    /// headers.insert(HeaderName::from_static("content-type"), "application/json".to_string());
    /// let response = rupring::Response::new().headers(headers);
    /// assert_eq!(response.headers.get(&HeaderName::from_static("content-type")).unwrap(), &"application/json".to_string());
    pub fn headers(mut self, headers: HashMap<HeaderName, String>) -> Self {
        self.headers = headers;
        return self;
    }

    /// redirect to url.
    /// ```
    /// use rupring::HeaderName;
    /// use std::collections::HashMap;
    /// let response = rupring::Response::new().redirect("https://naver.com".to_string());
    /// assert_eq!(response.headers.get(&HeaderName::from_static("location")).unwrap(), &"https://naver.com".to_string());
    pub fn redirect(mut self, url: String) -> Self {
        if self.status < 300 || self.status > 308 {
            self.status = 302;
        }

        self.header("location", url)
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
