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
