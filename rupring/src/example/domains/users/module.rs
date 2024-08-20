use super::controller::UserController;

#[derive(Debug, Clone, Copy)]
#[rupring::Module(
    controllers=[UserController{}],
    modules=[],
    providers=[
    ],
    middlewares=[]
)]
pub struct UserModule {}
