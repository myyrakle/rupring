//! Cross-Origin Resource Sharing (CORS) middleware.
//!
//! This module provides a small, dependency-free CORS middleware for Rupring.
//! It can be registered like any other middleware on a module or controller.
//!
//! For the common "allow every origin" case, register
//! [`default_cors_middleware`] directly:
//!
//! ```rust,ignore
//! #[derive(Debug, Clone, Copy)]
//! #[rupring::Module(
//!     controllers=[RootController{}],
//!     modules=[],
//!     providers=[],
//!     middlewares=[rupring::middleware::cors::default_cors_middleware],
//! )]
//! pub struct RootModule {}
//! ```
//!
//! For custom policy, build a [`Cors`] value and expose it from your own
//! middleware function:
//!
//! ```rust,ignore
//! pub fn cors_middleware(
//!     request: rupring::Request,
//!     response: rupring::Response,
//!     next: rupring::NextFunction,
//! ) -> rupring::Response {
//!     rupring::middleware::cors::Cors::new()
//!         .allow_origin("https://example.com")
//!         .allow_methods(["GET", "POST"])
//!         .allow_headers(["content-type", "authorization"])
//!         .max_age(3600)
//!         .middleware()(request, response, next)
//! }
//! ```
//!
//! Rupring handles browser preflight requests automatically when this
//! middleware is present in the matching route's middleware chain. A valid
//! preflight request is an `OPTIONS` request with `Origin` and
//! `Access-Control-Request-Method` headers.

use crate::{header, MiddlewareFunction, Request, Response};

/// Builder for configuring CORS response headers.
///
/// `Cors` is cloneable and can be used to create middleware with
/// [`Cors::middleware`]. The default policy allows any origin, common HTTP
/// methods, and reflects requested headers on preflight requests when allowed
/// headers are not explicitly configured.
///
/// The default policy is intentionally permissive for quick development and
/// examples. Production applications should usually set explicit origins and
/// headers.
#[derive(Debug, Clone)]
pub struct Cors {
    allow_any_origin: bool,
    allowed_origins: Vec<String>,
    allowed_methods: Vec<String>,
    allowed_headers: Option<Vec<String>>,
    exposed_headers: Vec<String>,
    allow_credentials: bool,
    max_age: Option<u64>,
}

impl Default for Cors {
    /// Creates a permissive CORS policy.
    ///
    /// Defaults:
    ///
    /// - `Access-Control-Allow-Origin: *`
    /// - allowed methods: `GET`, `POST`, `PUT`, `PATCH`, `DELETE`, `OPTIONS`,
    ///   `HEAD`
    /// - requested preflight headers are reflected when no explicit allowed
    ///   headers are configured
    /// - credentials are disabled
    /// - no `Access-Control-Max-Age`
    fn default() -> Self {
        Self {
            allow_any_origin: true,
            allowed_origins: Vec::new(),
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "PATCH".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
                "HEAD".to_string(),
            ],
            allowed_headers: None,
            exposed_headers: Vec::new(),
            allow_credentials: false,
            max_age: None,
        }
    }
}

impl Cors {
    /// Creates a new CORS builder with the same values as [`Cors::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Allows requests from any origin.
    ///
    /// This sets `Access-Control-Allow-Origin` to `*` unless credentials are
    /// enabled with [`Cors::allow_credentials`]. Browsers reject
    /// `Access-Control-Allow-Origin: *` when credentials are allowed, so in
    /// that case this middleware echoes the request `Origin` value instead.
    pub fn allow_any_origin(mut self) -> Self {
        self.allow_any_origin = true;
        self.allowed_origins.clear();
        self
    }

    /// Allows a single origin.
    ///
    /// When configured, CORS headers are only added if the request `Origin`
    /// exactly matches this value. Non-matching origins receive the unchanged
    /// response, leaving the browser to block cross-origin access.
    pub fn allow_origin(mut self, origin: impl ToString) -> Self {
        self.allow_any_origin = false;
        self.allowed_origins.push(origin.to_string());
        self
    }

    /// Allows a list of origins.
    ///
    /// Each origin is compared exactly against the request `Origin` header.
    /// This replaces any origins configured by earlier calls to
    /// [`Cors::allow_origin`] or [`Cors::allow_origins`].
    pub fn allow_origins<I, S>(mut self, origins: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.allow_any_origin = false;
        self.allowed_origins = origins
            .into_iter()
            .map(|origin| origin.to_string())
            .collect();
        self
    }

    /// Sets methods allowed for preflight requests.
    ///
    /// Values are uppercased and emitted as `Access-Control-Allow-Methods`
    /// when handling preflight requests. This method replaces the default
    /// method list.
    pub fn allow_methods<I, S>(mut self, methods: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.allowed_methods = methods
            .into_iter()
            .map(|method| method.to_string().to_uppercase())
            .collect();
        self
    }

    /// Sets request headers allowed for preflight requests.
    ///
    /// Values are emitted as `Access-Control-Allow-Headers`. If this method is
    /// not called, the middleware reflects the browser's
    /// `Access-Control-Request-Headers` value during preflight.
    pub fn allow_headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.allowed_headers = Some(
            headers
                .into_iter()
                .map(|header| header.to_string())
                .collect(),
        );
        self
    }

    /// Sets response headers exposed to browser JavaScript.
    ///
    /// Values are emitted as `Access-Control-Expose-Headers` on non-preflight
    /// responses. This is useful for custom response headers that browser code
    /// needs to read.
    pub fn expose_headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.exposed_headers = headers
            .into_iter()
            .map(|header| header.to_string())
            .collect();
        self
    }

    /// Enables or disables credentialed cross-origin requests.
    ///
    /// When `true`, the middleware emits
    /// `Access-Control-Allow-Credentials: true`. If any origin is allowed, the
    /// middleware echoes the request `Origin` instead of returning `*`, because
    /// credentialed browser requests cannot use wildcard origins.
    pub fn allow_credentials(mut self, allow_credentials: bool) -> Self {
        self.allow_credentials = allow_credentials;
        self
    }

    /// Sets the preflight cache duration in seconds.
    ///
    /// When configured, preflight responses include
    /// `Access-Control-Max-Age: <seconds>`.
    pub fn max_age(mut self, seconds: u64) -> Self {
        self.max_age = Some(seconds);
        self
    }

    /// Converts this policy into a Rupring middleware function.
    ///
    /// Normal requests are forwarded with `next` and receive CORS headers on
    /// the returned response. Preflight requests short-circuit with status
    /// `204` and CORS preflight headers.
    pub fn middleware(self) -> MiddlewareFunction {
        Box::new(move |request, response, next| {
            if is_preflight_request(&request) {
                return self.apply_preflight_headers(request, response.status(204));
            }

            let response = next(request.clone(), response);
            self.apply_headers(&request, response, false)
        })
    }

    /// Applies preflight headers to a response.
    ///
    /// This helper is public for framework internals and focused tests. Most
    /// applications should use [`Cors::middleware`] or
    /// [`default_cors_middleware`] instead.
    pub fn apply_preflight_headers(&self, request: Request, response: Response) -> Response {
        self.apply_headers(&request, response.status(204), true)
    }

    fn apply_headers(
        &self,
        request: &Request,
        mut response: Response,
        preflight: bool,
    ) -> Response {
        let allowed_origin = match self.resolve_allowed_origin(request) {
            Some(origin) => origin,
            None => return response,
        };

        response = response.header(header::ACCESS_CONTROL_ALLOW_ORIGIN, allowed_origin);

        if !self.allow_any_origin || self.allow_credentials {
            response = response.header(header::VARY, "Origin");
        }

        if self.allow_credentials {
            response = response.header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true");
        }

        if preflight {
            if !self.allowed_methods.is_empty() {
                response = response.header(
                    header::ACCESS_CONTROL_ALLOW_METHODS,
                    self.allowed_methods.join(", "),
                );
            }

            if let Some(allowed_headers) = &self.allowed_headers {
                if !allowed_headers.is_empty() {
                    response = response.header(
                        header::ACCESS_CONTROL_ALLOW_HEADERS,
                        allowed_headers.join(", "),
                    );
                }
            } else if let Some(request_headers) =
                request.headers.get(header::ACCESS_CONTROL_REQUEST_HEADERS)
            {
                response = response.header(
                    header::ACCESS_CONTROL_ALLOW_HEADERS,
                    request_headers.clone(),
                );
            }

            if let Some(max_age) = self.max_age {
                response = response.header(header::ACCESS_CONTROL_MAX_AGE, max_age);
            }
        } else if !self.exposed_headers.is_empty() {
            response = response.header(
                header::ACCESS_CONTROL_EXPOSE_HEADERS,
                self.exposed_headers.join(", "),
            );
        }

        response
    }

    fn resolve_allowed_origin(&self, request: &Request) -> Option<String> {
        let request_origin = request.headers.get(header::ORIGIN);

        if self.allow_any_origin {
            if self.allow_credentials {
                return request_origin.cloned();
            }

            return Some("*".to_string());
        }

        let request_origin = request_origin?;
        if self
            .allowed_origins
            .iter()
            .any(|origin| origin == request_origin)
        {
            return Some(request_origin.clone());
        }

        None
    }
}

/// Returns `true` when a request is a CORS preflight request.
///
/// A request is treated as preflight when it uses the `OPTIONS` method and has
/// both `Origin` and `Access-Control-Request-Method` headers.
pub fn is_preflight_request(request: &Request) -> bool {
    request.method == crate::Method::OPTIONS
        && request.headers.contains_key(header::ORIGIN)
        && request
            .headers
            .contains_key(header::ACCESS_CONTROL_REQUEST_METHOD)
}

/// Default CORS middleware function for direct registration.
///
/// This is a convenience wrapper around `Cors::default().middleware()`. It is
/// useful in Rupring macro attributes because middleware registration expects a
/// function path.
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Copy)]
/// #[rupring::Module(
///     controllers=[RootController{}],
///     modules=[],
///     providers=[],
///     middlewares=[rupring::middleware::cors::default_cors_middleware],
/// )]
/// pub struct RootModule {}
/// ```
pub fn default_cors_middleware(
    request: Request,
    response: Response,
    next: crate::NextFunction,
) -> Response {
    Cors::default().middleware()(request, response, next)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HeaderName, Request, Response};
    use std::{collections::HashMap, net::IpAddr, sync::Arc};

    fn request(method: crate::Method, origin: Option<&str>) -> Request {
        let mut headers = HashMap::new();
        if let Some(origin) = origin {
            headers.insert("origin".to_string(), origin.to_string());
        }

        Request {
            method,
            path: "/hello".to_string(),
            body: String::new(),
            raw_body: Vec::new(),
            files: Vec::new(),
            headers,
            cookies: HashMap::new(),
            query_parameters: HashMap::new(),
            path_parameters: HashMap::new(),
            metadata: crate::request::Metadata {
                ip: IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                ..Default::default()
            },
            di_context: Arc::new(crate::DIContext::new()),
        }
    }

    fn next(_: Request, response: Response) -> Response {
        response.text("ok")
    }

    fn header_values(response: &Response, name: &'static str) -> Option<Vec<String>> {
        response
            .headers
            .get(&HeaderName::from_static(name))
            .cloned()
    }

    #[test]
    fn default_cors_adds_any_origin_after_next() {
        let middleware = Cors::default().middleware();

        let response = middleware(
            request(crate::Method::GET, Some("https://example.com")),
            Response::new(),
            next,
        );

        assert_eq!(
            header_values(&response, "access-control-allow-origin"),
            Some(vec!["*".to_string()])
        );
        assert_eq!(response.data.into_bytes(), b"ok".to_vec());
    }

    #[test]
    fn explicit_origins_only_allow_matching_origin() {
        let middleware = Cors::new()
            .allow_origin("https://allowed.example")
            .middleware();

        let response = middleware(
            request(crate::Method::GET, Some("https://blocked.example")),
            Response::new(),
            next,
        );

        assert_eq!(
            header_values(&response, "access-control-allow-origin"),
            None
        );
    }

    #[test]
    fn preflight_reflects_requested_headers_when_allowed_headers_are_not_configured() {
        let mut request = request(crate::Method::OPTIONS, Some("https://example.com"));
        request.headers.insert(
            "access-control-request-headers".to_string(),
            "x-api-key, authorization".to_string(),
        );

        let response = Cors::default().apply_preflight_headers(request, Response::new());

        assert_eq!(
            header_values(&response, "access-control-allow-headers"),
            Some(vec!["x-api-key, authorization".to_string()])
        );
    }

    #[test]
    fn default_cors_middleware_function_can_be_registered_directly() {
        let response = default_cors_middleware(
            request(crate::Method::GET, Some("https://example.com")),
            Response::new(),
            next,
        );

        assert_eq!(
            header_values(&response, "access-control-allow-origin"),
            Some(vec!["*".to_string()])
        );
    }
}
