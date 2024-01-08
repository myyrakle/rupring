use super::controller::SwaggerController;
use crate as rupring;

use super::context::InjectSwaggerContext;

#[derive(Debug, Clone)]
#[rupring_macro::Module(
    controllers = SwaggerController{}, 
    providers = [InjectSwaggerContext{}]
)]
pub struct SwaggerModule {}
