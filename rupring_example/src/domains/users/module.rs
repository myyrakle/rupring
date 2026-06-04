use super::controller::UserController;
use super::repository::inject_user_repository;
use super::service::inject_user_service;

#[derive(Debug, Clone, Copy)]
#[rupring::Module(
    controllers=[UserController{}],
    modules=[],
    providers=[
        inject_user_service{},
        inject_user_repository{},
    ],
    middlewares=[]
)]
pub struct UserModule {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::domains::users::interface::IUserService;

    #[test]
    fn user_module_registers_user_service_as_arc_trait_object() {
        let mut di_context = rupring::DIContext::new();
        di_context.initialize(Box::new(UserModule {}));

        assert!(di_context.get::<Arc<dyn IUserService>>().is_some());
    }
}
