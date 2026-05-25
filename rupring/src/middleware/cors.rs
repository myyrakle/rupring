use crate::{header, MiddlewareFunction, Request, Response};

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_any_origin(mut self) -> Self {
        self.allow_any_origin = true;
        self.allowed_origins.clear();
        self
    }

    pub fn allow_origin(mut self, origin: impl ToString) -> Self {
        self.allow_any_origin = false;
        self.allowed_origins.push(origin.to_string());
        self
    }

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

    pub fn allow_credentials(mut self, allow_credentials: bool) -> Self {
        self.allow_credentials = allow_credentials;
        self
    }

    pub fn max_age(mut self, seconds: u64) -> Self {
        self.max_age = Some(seconds);
        self
    }

    pub fn middleware(self) -> MiddlewareFunction {
        Box::new(move |request, response, next| {
            if is_preflight_request(&request) {
                return self.apply_preflight_headers(request, response.status(204));
            }

            let response = next(request.clone(), response);
            self.apply_headers(&request, response, false)
        })
    }

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

pub fn is_preflight_request(request: &Request) -> bool {
    request.method == crate::Method::OPTIONS
        && request.headers.contains_key(header::ORIGIN)
        && request
            .headers
            .contains_key(header::ACCESS_CONTROL_REQUEST_METHOD)
}

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
