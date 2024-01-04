use super::routes;
use crate as rupring;

#[derive(Debug, Clone)]
#[rupring_macro::Controller(
    prefix = /docs, 
    routes=[
        routes::get_docs, 
        routes::get_css,
    ]
)]
pub struct SwaggerController {}
