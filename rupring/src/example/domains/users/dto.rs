use rupring_macro::RupringDoc;
use serde::{Deserialize, Serialize};

pub mod foo {

    use rupring_macro::RupringDoc;
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize, RupringDoc)]
    pub struct Bar {
        pub a: i32,
        pub b: String,
    }
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
    #[example = 1]
    #[required = false]
    pub foo: i32,
    #[example = true]
    #[required]
    pub asdf: bool,
    #[desc = "설명"]
    pub ids: Vec<i32>,

    #[query = "query_test111"]
    #[desc = "설명"]
    #[example = "asdf"]
    pub query_test: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {}

#[derive(Debug, Serialize, Deserialize, RupringDoc)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing)]
    #[path_param = "id"]
    #[desc = "user 고유 id"]
    #[required]
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

#[derive(Debug, Serialize, Deserialize, RupringDoc)]
pub struct GetUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, RupringDoc)]
pub struct ListUsersRequest {
    #[query = "offset"]
    pub offset: i32,
    #[query = "limit"]
    pub limit: i32,
}

#[derive(Debug, Serialize, Deserialize, RupringDoc)]
pub struct ListUsersResponse {
    pub users: Vec<GetUserResponse>,
    pub total: i32,
}
