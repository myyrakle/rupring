use super::routes;
use crate as rupring;

#[derive(Debug, Clone)]
#[rupring_macro::Controller(
    prefix = /,
    routes=[
        routes::get_openapi_json,
    ]
)]
pub struct OpenApiController {}
