use rupring::RupringDto;
use serde::{Deserialize, Serialize};

pub mod foo {
    use rupring::RupringDto;
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize, RupringDto)]
    pub struct Bar {
        pub a: i32,
        pub b: String,
    }
}

#[derive(Debug, Serialize, Deserialize, RupringDto)]
pub struct CreateUserRequest {
    #[desc = "user name"]
    #[example = "foobar"]
    pub username: String,

    pub email: String,

    #[desc = "user password"]
    #[example = "q1w2e3r4"]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {}

#[derive(Debug, Serialize, Deserialize, RupringDto)]
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

#[derive(Debug, Serialize, Deserialize, RupringDto)]
pub struct GetUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, RupringDto)]
pub struct ListUsersRequest {
    #[query = "offset"]
    pub offset: i32,
    #[query = "limit"]
    pub limit: i32,
}

#[derive(Debug, Serialize, Deserialize, RupringDto)]
pub struct ListUsersResponse {
    pub users: Vec<GetUserResponse>,
    pub total: i32,
}
