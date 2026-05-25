use std::collections::HashMap;

// standard headers
pub const ACCEPT: &str = "accept";
pub const ACCEPT_ENCODING: &str = "accept-encoding";
pub const ACCEPT_LANGUAGE: &str = "accept-language";
pub const ACCEPT_CHARSET: &str = "accept-charset";
pub const ACCEPT_DATETIME: &str = "accept-datetime";
pub const CONTENT_TYPE: &str = "content-type";
pub const CONTENT_LENGTH: &str = "content-length";
pub const CONTENT_ENCODING: &str = "content-encoding";
pub const USER_AGENT: &str = "user-agent";
pub const HOST: &str = "host";
pub const CONNECTION: &str = "connection";
pub const SET_COOKIE: &str = "set-cookie";
pub const CONTENT_DISPOSITION: &str = "content-disposition";
pub const CACHE_CONTROL: &str = "cache-control";
pub const COOKIE: &str = "cookie";
pub const ACCESS_CONTROL_ALLOW_ORIGIN: &str = "access-control-allow-origin";
pub const ACCESS_CONTROL_ALLOW_METHODS: &str = "access-control-allow-methods";
pub const ACCESS_CONTROL_ALLOW_HEADERS: &str = "access-control-allow-headers";
pub const ACCESS_CONTROL_ALLOW_CREDENTIALS: &str = "access-control-allow-credentials";
pub const ACCESS_CONTROL_EXPOSE_HEADERS: &str = "access-control-expose-headers";
pub const ACCESS_CONTROL_MAX_AGE: &str = "access-control-max-age";
pub const ACCESS_CONTROL_REQUEST_METHOD: &str = "access-control-request-method";
pub const ACCESS_CONTROL_REQUEST_HEADERS: &str = "access-control-request-headers";
pub const ORIGIN: &str = "origin";
pub const VARY: &str = "vary";
pub const KEEP_ALIVE: &str = "keep-alive";

// response only headers
pub const LOCATION: &str = "location";

// custum headers
pub const REQUEST_ID: &str = "request-id";

pub(crate) fn preprocess_headers(header: &mut HashMap<String, String>) {
    if !header.contains_key(REQUEST_ID) {
        header.insert(REQUEST_ID.to_string(), uuid::Uuid::new_v4().to_string());
    }
}
