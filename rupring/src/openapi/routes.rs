use crate::{self as rupring, header, http::meme};

#[rupring_macro::GetMapping(path = /openapi.json)]
pub fn get_openapi_json(request: rupring::Request) -> rupring::Response {
    let openapi_context = request
        .di_context
        .get::<super::context::OpenApiContext>()
        .unwrap();

    let json = openapi_context.openapi_json.read().unwrap().to_owned();

    rupring::Response::new()
        .text(json)
        .header(header::CONTENT_TYPE, meme::JAVASCRIPT)
}
