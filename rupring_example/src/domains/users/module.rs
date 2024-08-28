use super::controller::UserController;
use super::service::inject_user_service;
use super::repository::inject_user_repository;

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
