use super::routes;
use crate as rupring;

#[derive(Debug, Clone)]
#[rupring_macro::Controller(
    prefix = /docs, 
    routes=[
        routes::get_docs, 
        routes::get_favicon32,
        routes::get_favicon16,
        routes::get_json,
        routes::get_swagger_ui_bundle,
        routes::get_swagger_ui_css,
    ]
)]
pub struct SwaggerController {}
