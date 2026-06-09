use crate::{self as rupring, header, http::meme};

#[rupring_macro::GetMapping(path = /)]
pub fn get_docs(_: rupring::Request) -> rupring::Response {
    rupring::Response::new()
        .text(super::html::DOCS_INDEX_HTML)
        .header(header::CONTENT_TYPE, meme::HTML)
}
