use super::controller::SwaggerController;
use crate as rupring;

#[derive(Debug, Clone)]
#[rupring_macro::Module(controllers = SwaggerController{})]
pub struct SwaggerModule {}
