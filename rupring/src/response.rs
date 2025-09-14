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

use crate::{header, http::meme, HeaderName, Request};
use http_body_util::Full;
use hyper::body::Bytes;

/// HTTP cookie
#[derive(Debug, Clone, Default)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub expires: Option<String>,
    pub max_age: Option<String>,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: Option<bool>,
    pub http_only: Option<bool>,
    pub same_site: Option<String>,
}

impl Cookie {
    /// Create a new cookie.
    /// ```
    /// let cookie = rupring::response::Cookie::new("foo", "bar");
    /// assert_eq!(cookie.name, "foo");
    /// assert_eq!(cookie.value, "bar");
    /// ```
    pub fn new(name: impl ToString, value: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            ..Default::default()
        }
    }

    /// Set the expiration date of the cookie.
    /// ```
    /// let cookie = rupring::response::Cookie::new("foo", "bar").expires("Wed, 21 Oct 2015 07:28:00 GMT");
    /// assert_eq!(cookie.expires.unwrap(), "Wed, 21 Oct 2015 07:28:00 GMT");
    /// ```
    pub fn expires(mut self, expires: impl ToString) -> Self {
        self.expires = Some(expires.to_string());
        self
    }

    /// Set the maximum age of the cookie.
    /// ```
    /// let cookie = rupring::response::Cookie::new("foo", "bar").max_age("3600");
    /// assert_eq!(cookie.max_age.unwrap(), "3600");
    /// ```
    pub fn max_age(mut self, max_age: impl ToString) -> Self {
        self.max_age = Some(max_age.to_string());
        self
    }

    /// Set the domain of the cookie.
    /// ```
    /// let cookie = rupring::response::Cookie::new("foo", "bar").domain("example.com");
    /// assert_eq!(cookie.domain.unwrap(), "example.com");
    /// ```
    pub fn domain(mut self, domain: impl ToString) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    /// Set the path of the cookie.
    /// ```
    /// let cookie = rupring::response::Cookie::new("foo", "bar").path("/path");
    /// assert_eq!(cookie.path.unwrap(), "/path");
    /// ```
    pub fn path(mut self, path: impl ToString) -> Self {
        self.path = Some(path.to_string());
        self
    }

    /// Set the secure flag of the cookie.
    /// ```
    /// let cookie = rupring::response::Cookie::new("foo", "bar").secure(true);
    /// assert_eq!(cookie.secure.unwrap(), true);
    /// ```
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = Some(secure);
        self
    }

    /// Set the http only flag of the cookie.
    /// ```
    /// let cookie = rupring::response::Cookie::new("foo", "bar").http_only(true);
    /// assert_eq!(cookie.http_only.unwrap(), true);
    /// ```
    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = Some(http_only);
        self
    }

    /// Set the same site attribute of the cookie.
    /// ```
    /// let cookie = rupring::response::Cookie::new("foo", "bar").same_site("Strict");
    /// assert_eq!(cookie.same_site.unwrap(), "Strict");
    /// ```
    pub fn same_site(mut self, same_site: impl ToString) -> Self {
        self.same_site = Some(same_site.to_string());
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
    pub headers: HashMap<HeaderName, Vec<String>>,
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
            body: Vec::new(),
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
    /// assert_eq!(response.body, r#"{"name":"John"}"#.to_string().into_bytes());
    /// // ...
    /// ```
    pub fn json(mut self, body: impl serde::Serialize) -> Self {
        self.headers.insert(
            crate::HeaderName::from_static(header::CONTENT_TYPE),
            vec![meme::JSON.into()],
        );

        self.body = match serde_json::to_string(&body) {
            Ok(body) => body,
            Err(err) => {
                self.status = 500;
                format!("Error serializing response body: {:?}", err)
            }
        }
        .into();

        self
    }

    /// Set to return a text value.
    /// ```
    /// let response = rupring::Response::new().text("Hello World".to_string());
    /// assert_eq!(response.body, "Hello World".to_string().into_bytes());
    pub fn text(mut self, body: impl ToString) -> Self {
        self.headers.insert(
            crate::HeaderName::from_static(header::CONTENT_TYPE),
            vec![meme::TEXT.to_string()],
        );

        self.body = body.to_string().into();

        self
    }

    /// set to return a html value.
    /// ```
    /// let response = rupring::Response::new().html("<h1>Hello World</h1>".to_string());
    /// assert_eq!(response.body, "<h1>Hello World</h1>".to_string().into_bytes());
    /// ```
    pub fn html(mut self, body: impl ToString) -> Self {
        self.headers.insert(
            crate::HeaderName::from_static(header::CONTENT_TYPE),
            vec![meme::HTML.to_string()],
        );

        self.body = body.to_string().into();

        self
    }

    /// Set `Content-Diposition` header to cause the browser to download the file.
    /// ```
    /// use rupring::HeaderName;
    ///
    /// let response = rupring::Response::new().download("hello.txt", "Hello World");
    /// assert_eq!(response.headers.get(&HeaderName::from_static("content-disposition")).unwrap(), &vec!["attachment; filename=\"hello.txt\"".to_string()]);
    /// assert_eq!(response.body, "Hello World".to_string().into_bytes());
    /// ```
    pub fn download(mut self, filename: impl ToString, file: impl Into<Vec<u8>>) -> Self {
        self.headers.insert(
            crate::HeaderName::from_static(header::CONTENT_DISPOSITION),
            vec![format!("attachment; filename=\"{}\"", filename.to_string())],
        );

        self.body = file.into();

        self
    }

    /// Set the cache control header for browser caching.
    /// ```
    /// use rupring::HeaderName;
    ///
    /// let response = rupring::Response::new().cache_control(rupring::response::CacheControl {
    ///   max_age: Some(3600),
    ///  s_max_age: Some(3800),
    ///  ..Default::default()
    /// });
    /// assert_eq!(response.headers.get(&HeaderName::from_static("cache-control")).unwrap(), &vec!["max-age=3600, s-maxage=3800".to_string()]);
    /// ```
    pub fn cache_control(mut self, cache_control: crate::http::cache::CacheControl) -> Self {
        let mut cache_control_str = String::new();

        if let Some(max_age) = cache_control.max_age {
            cache_control_str.push_str(&format!("max-age={}", max_age));
        }

        if let Some(s_maxage) = cache_control.s_max_age {
            if !cache_control_str.is_empty() {
                cache_control_str.push_str(", ");
            }

            cache_control_str.push_str(&format!("s-maxage={}", s_maxage));
        }

        if cache_control.private {
            if !cache_control_str.is_empty() {
                cache_control_str.push_str(", ");
            }

            cache_control_str.push_str("private");
        }

        if cache_control.no_cache {
            if !cache_control_str.is_empty() {
                cache_control_str.push_str(", ");
            }

            cache_control_str.push_str("no-cache");
        }

        if cache_control.no_store {
            if !cache_control_str.is_empty() {
                cache_control_str.push_str(", ");
            }

            cache_control_str.push_str("no-store");
        }

        if cache_control.no_transform {
            if !cache_control_str.is_empty() {
                cache_control_str.push_str(", ");
            }

            cache_control_str.push_str("no-transform");
        }

        if cache_control.must_revalidate {
            if !cache_control_str.is_empty() {
                cache_control_str.push_str(", ");
            }

            cache_control_str.push_str("must-revalidate");
        }

        if cache_control.proxy_revalidate {
            if !cache_control_str.is_empty() {
                cache_control_str.push_str(", ");
            }

            cache_control_str.push_str("proxy-revalidate");
        }

        if cache_control.immutable {
            if !cache_control_str.is_empty() {
                cache_control_str.push_str(", ");
            }

            cache_control_str.push_str("immutable");
        }

        if let Some(stale_while_revalidate) = cache_control.stale_while_revalidate {
            if !cache_control_str.is_empty() {
                cache_control_str.push_str(", ");
            }

            cache_control_str.push_str(&format!(
                "stale-while-revalidate={}",
                stale_while_revalidate
            ));
        }

        if let Some(stale_if_error) = cache_control.stale_if_error {
            if !cache_control_str.is_empty() {
                cache_control_str.push_str(", ");
            }

            cache_control_str.push_str(&format!("stale-if-error={}", stale_if_error));
        }

        self.headers.insert(
            HeaderName::from_static(header::CACHE_CONTROL),
            vec![cache_control_str],
        );

        self
    }

    /// Set status code.
    /// ```
    /// let response = rupring::Response::new().status(404);
    /// assert_eq!(response.status, 404);
    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }

    /// Set a header.
    /// ```
    /// use rupring::HeaderName;
    /// let response = rupring::Response::new().header("content-type", "application/json".to_string());
    /// assert_eq!(response.headers.get(&HeaderName::from_static("content-type")).unwrap(), &vec!["application/json".to_string()]);
    pub fn header(mut self, name: &'static str, value: impl ToString) -> Self {
        if let Some(values) = self.headers.get_mut(&HeaderName::from_static(name)) {
            values.push(value.to_string());
        } else {
            self.headers
                .insert(HeaderName::from_static(name), vec![value.to_string()]);
        }

        self
    }

    /// overwrite headers.
    /// ```
    /// use rupring::HeaderName;
    /// use std::collections::HashMap;
    /// let mut headers = HashMap::new();
    /// headers.insert(HeaderName::from_static("content-type"), vec!["application/json".to_string()]);
    /// let response = rupring::Response::new().headers(headers);
    /// assert_eq!(response.headers.get(&HeaderName::from_static("content-type")).unwrap(), &vec!["application/json".to_string()]);
    pub fn headers(mut self, headers: HashMap<HeaderName, Vec<String>>) -> Self {
        self.headers = headers;
        self
    }

    /// redirect to url.
    /// ```
    /// use rupring::HeaderName;
    /// use std::collections::HashMap;
    /// let response = rupring::Response::new().redirect("https://naver.com");
    /// assert_eq!(response.headers.get(&HeaderName::from_static("location")).unwrap(), &vec!["https://naver.com".to_string()]);
    pub fn redirect(mut self, url: impl ToString) -> Self {
        if self.status < 300 || self.status > 308 {
            self.status = 302;
        }

        self.header(header::LOCATION, url)
    }

    /// add a cookie to the response.
    /// ```
    /// use rupring::HeaderName;
    /// use rupring::response::Cookie;
    /// let response = rupring::Response::new().add_cookie(Cookie::new("foo", "bar"));
    /// assert_eq!(response.headers.get(&HeaderName::from_static("set-cookie")).unwrap(), &vec!["foo=bar".to_string()]);
    /// ```
    pub fn add_cookie(mut self, cookie: Cookie) -> Self {
        let mut cookie_str = format!("{}={}", cookie.name, cookie.value);

        if let Some(expires) = cookie.expires {
            cookie_str.push_str(&format!("; Expires={}", expires));
        }

        if let Some(max_age) = cookie.max_age {
            cookie_str.push_str(&format!("; Max-Age={}", max_age));
        }

        if let Some(domain) = cookie.domain {
            cookie_str.push_str(&format!("; Domain={}", domain));
        }

        if let Some(path) = cookie.path {
            cookie_str.push_str(&format!("; Path={}", path));
        }

        if let Some(secure) = cookie.secure {
            cookie_str.push_str(&format!("; Secure={}", secure));
        }

        if let Some(http_only) = cookie.http_only {
            cookie_str.push_str(&format!("; HttpOnly={}", http_only));
        }

        if let Some(same_site) = cookie.same_site {
            cookie_str.push_str(&format!("; SameSite={}", same_site));
        }

        self.headers
            .entry(HeaderName::from_static(header::SET_COOKIE))
            .or_default()
            .push(cookie_str);

        self
    }
}

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl From<Response> for hyper::Response<Full<Bytes>> {
    fn from(response: Response) -> Self {
        let mut builder = hyper::Response::builder();

        builder = builder.status(response.status);

        for (header_name, header_values) in response.headers {
            for header_value in header_values {
                builder = builder.header(header_name.clone(), header_value);
            }
        }

        builder.body(Full::new(Bytes::from(response.body))).unwrap()
    }
}
