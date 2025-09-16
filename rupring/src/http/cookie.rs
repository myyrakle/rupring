use std::collections::HashMap;

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
    /// let cookie = rupring::http::cookie::Cookie::new("foo", "bar");
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
    /// let cookie = rupring::http::cookie::Cookie::new("foo", "bar").expires("Wed, 21 Oct 2015 07:28:00 GMT");
    /// assert_eq!(cookie.expires.unwrap(), "Wed, 21 Oct 2015 07:28:00 GMT");
    /// ```
    pub fn expires(mut self, expires: impl ToString) -> Self {
        self.expires = Some(expires.to_string());
        self
    }

    /// Set the maximum age of the cookie.
    /// ```
    /// let cookie = rupring::http::cookie::Cookie::new("foo", "bar").max_age("3600");
    /// assert_eq!(cookie.max_age.unwrap(), "3600");
    /// ```
    pub fn max_age(mut self, max_age: impl ToString) -> Self {
        self.max_age = Some(max_age.to_string());
        self
    }

    /// Set the domain of the cookie.
    /// ```
    /// let cookie = rupring::http::cookie::Cookie::new("foo", "bar").domain("example.com");
    /// assert_eq!(cookie.domain.unwrap(), "example.com");
    /// ```
    pub fn domain(mut self, domain: impl ToString) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    /// Set the path of the cookie.
    /// ```
    /// let cookie = rupring::http::cookie::Cookie::new("foo", "bar").path("/path");
    /// assert_eq!(cookie.path.unwrap(), "/path");
    /// ```
    pub fn path(mut self, path: impl ToString) -> Self {
        self.path = Some(path.to_string());
        self
    }

    /// Set the secure flag of the cookie.
    /// ```
    /// let cookie = rupring::http::cookie::Cookie::new("foo", "bar").secure(true);
    /// assert_eq!(cookie.secure.unwrap(), true);
    /// ```
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = Some(secure);
        self
    }

    /// Set the http only flag of the cookie.
    /// ```
    /// let cookie = rupring::http::cookie::Cookie::new("foo", "bar").http_only(true);
    /// assert_eq!(cookie.http_only.unwrap(), true);
    /// ```
    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = Some(http_only);
        self
    }

    /// Set the same site attribute of the cookie.
    /// ```
    /// let cookie = rupring::http::cookie::Cookie::new("foo", "bar").same_site("Strict");
    /// assert_eq!(cookie.same_site.unwrap(), "Strict");
    /// ```
    pub fn same_site(mut self, same_site: impl ToString) -> Self {
        self.same_site = Some(same_site.to_string());
        self
    }
}

pub(crate) fn parse_cookie_header(cookie_header: &str) -> HashMap<String, String> {
    let mut cookies = HashMap::new();

    for cookie in cookie_header.split("; ") {
        let mut parts = cookie.splitn(2, '=');
        if let Some(key) = parts.next() {
            if let Some(value) = parts.next() {
                cookies.insert(key.to_string(), value.to_string());
            }
        }
    }

    cookies
}
