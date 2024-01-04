use crate as rupring;

#[derive(Debug, Clone)]
#[rupring_macro::Controller(prefix = /docs, routes=[super::routes::get_docs])]
pub struct SwaggerController {}
