use super::controller::RootController;
use crate::middlewares::logger::logger_middleware;
use crate::domains::users::module::UserModule;
use rupring::swagger::module::SwaggerModule;

#[derive(Debug, Clone, Copy)]
#[rupring::Module(
    controllers=[RootController{}], 
    modules=[UserModule{}, SwaggerModule{}], 
    providers=[], 
    middlewares=[logger_middleware],
)]
pub struct RootModule {}
