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

// response only headers
pub const LOCATION: &str = "location";

// custum headers
pub const REQUEST_ID: &str = "request-id";

pub(crate) fn preprocess_headers(header: &mut HashMap<String, String>) {
    if !header.contains_key(REQUEST_ID) {
        header.insert(REQUEST_ID.to_string(), uuid::Uuid::new_v4().to_string());
    }
}
