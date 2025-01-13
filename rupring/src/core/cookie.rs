use std::collections::HashMap;

pub fn parse_cookie_header(cookie_header: &str) -> HashMap<String, String> {
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
