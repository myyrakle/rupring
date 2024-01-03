use crate as rupring;

#[derive(Debug, Clone)]
#[rupring_macro::Controller(prefix = /docs)]
pub struct SwaggerController {}
