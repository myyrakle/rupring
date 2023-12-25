use std::collections::HashMap;

pub const REQUEST_ID: &'static str = "request-id";
// pub const CONTENT_TYPE: &'static str = "content-type";
// pub const CONTENT_LENGTH: &'static str = "content-length";
// pub const ACCEPT: &'static str = "accept";
// pub const ACCEPT_ENCODING: &'static str = "accept-encoding";
// pub const ACCEPT_LANGUAGE: &'static str = "accept-language";
// pub const ACCEPT_CHARSET: &'static str = "accept-charset";
// pub const ACCEPT_DATETIME: &'static str = "accept-datetime";

pub(crate) fn preprocess_headers(header: &mut HashMap<String, String>) {
    if !header.contains_key(REQUEST_ID) {
        header.insert(REQUEST_ID.to_string(), uuid::Uuid::new_v4().to_string());
    }
}
