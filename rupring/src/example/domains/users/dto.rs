use rupring_macro::RupringDoc;
use serde::{Deserialize, Serialize};

pub mod foo {

    use rupring_macro::RupringDoc;
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize, RupringDoc)]
    pub struct Bar {}
}

#[derive(Debug, Serialize, Deserialize, RupringDoc)]
pub struct CreateUserRequest {
    #[desc = "유저명"]
    #[example = "foobar"]
    pub username: String,
    pub email: String,
    #[desc = "비밀번호"]
    #[example = "password"]
    pub password: String,
    pub bar: foo::Bar,
    #[example = 1]
    pub foo: i32,
    #[example = true]
    pub asdf: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersRequest {
    pub offset: i32,
    pub limit: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersResponse {
    pub users: Vec<GetUserResponse>,
    pub total: i32,
}