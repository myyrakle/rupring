use super::controller::RootController;
use crate::domains::users::module::UserModule;
use crate::middlewares::logger::logger_middleware;
use rupring::middleware::cors::default_cors_middleware;
use rupring::scalar::module::ScalarModule;

#[derive(Debug, Clone, Copy)]
#[rupring::Module(
    controllers=[RootController{}],
    modules=[UserModule{}, ScalarModule{}],
    providers=[],
    middlewares=[logger_middleware, default_cors_middleware],
)]
pub struct RootModule {}
