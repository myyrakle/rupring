/*!
# About Reponse
- Response is a struct that represents the HTTP response to be returned to the client.

You can create a response like this:
```rust
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text("Hello, World!".to_string())
}
```

You can also return a json value like this:
```rust
#[derive(serde::Serialize)]
struct User {
    name: String,
}

#[rupring::Get(path = /user)]
pub fn get_user(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().json(User {
        name: "John".to_string(),
    })
}
```

You can set the status code like this:
```rust
#[rupring::Get(path = /asdf)]
pub fn not_found(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text("not found".to_string()).status(404)
}
```

You can set the header like this:
```rust
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new()
        .text("Hello, World!".to_string())
        .header("content-type", "text/plain".to_string())
}
```

If you want, you can receive it as a parameter instead of creating the response directly.
```rust
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request, response: rupring::Response) -> rupring::Response {
    response
        .text("Hello, World!".to_string())
        .header("content-type", "text/plain".to_string())
}
```
This is especially useful when you need to inherit and use Response through middleware.

If you want to redirect, you can use Response’s redirect method.
```rust
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().redirect("/hello")
}
```
This method automatically sets status to 302 unless you set it to 300-308.
*/

use std::{collections::HashMap, panic::UnwindSafe};

use crate::{header, meme, HeaderName, Request};
use http_body_util::Full;
use hyper::body::Bytes;

#[derive(Debug, Clone, Default)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub(crate) expires: Option<String>,
    pub(crate) max_age: Option<String>,
    pub(crate) domain: Option<String>,
    pub(crate) path: Option<String>,
    pub(crate) secure: Option<bool>,
    pub(crate) http_only: Option<bool>,
    pub(crate) same_site: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
    pub headers: HashMap<HeaderName, String>,
    pub(crate) next: Option<Box<(Request, Response)>>,
    pub(crate) set_cookies: Vec<Cookie>,
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
            body: Vec::new(),
            headers: Default::default(),
            next: None,
            set_cookies: Vec::new(),
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
    /// assert_eq!(response.body, r#"{"name":"John"}"#.to_string().into_bytes());
    /// // ...
    /// ```
    pub fn json(mut self, body: impl serde::Serialize) -> Self {
        self.headers.insert(
            crate::HeaderName::from_static(header::CONTENT_TYPE),
            meme::JSON.into(),
        );

        self.body = match serde_json::to_string(&body) {
            Ok(body) => body,
            Err(err) => {
                self.status = 500;
                format!("Error serializing response body: {:?}", err)
            }
        }
        .into();

        return self;
    }

    /// Set to return a text value.
    /// ```
    /// let response = rupring::Response::new().text("Hello World".to_string());
    /// assert_eq!(response.body, "Hello World".to_string().into_bytes());
    pub fn text(mut self, body: impl ToString) -> Self {
        self.headers.insert(
            crate::HeaderName::from_static(header::CONTENT_TYPE),
            meme::TEXT.to_string(),
        );

        self.body = body.to_string().into();

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
    pub fn header(mut self, name: &'static str, value: impl ToString) -> Self {
        self.headers
            .insert(HeaderName::from_static(name), value.to_string());
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
    /// let response = rupring::Response::new().redirect("https://naver.com");
    /// assert_eq!(response.headers.get(&HeaderName::from_static("location")).unwrap(), &"https://naver.com".to_string());
    pub fn redirect(mut self, url: impl ToString) -> Self {
        if self.status < 300 || self.status > 308 {
            self.status = 302;
        }

        self.header(header::LOCATION, url)
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
