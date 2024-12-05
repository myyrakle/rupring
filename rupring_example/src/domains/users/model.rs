#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub type Users = Vec<User>;

pub struct DeleteUserParams {
    pub id: i32,
}

pub struct GetUserParams {
    pub id: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateUserParams {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct UpdateUserParams {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct ListUsersParams {
    pub limit: i32,
    pub offset: i32,
}

pub struct CountUsersParams {}
