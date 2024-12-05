use super::{
    dto::{
        CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse,
        GetUserRequest, GetUserResponse, ListUsersRequest, ListUsersResponse, UpdateUserRequest,
        UpdateUserResponse,
    },
    model::{
        CountUsersParams, CreateUserParams, DeleteUserParams, GetUserParams, ListUsersParams,
        UpdateUserParams, User, Users,
    },
};

pub trait IUserService {
    fn create_user(&self, request: CreateUserRequest) -> rupring::Result<CreateUserResponse>;
    fn update_user(&self, request: UpdateUserRequest) -> rupring::Result<UpdateUserResponse>;
    fn delete_user(&self, request: DeleteUserRequest) -> rupring::Result<DeleteUserResponse>;
    fn get_user(&self, request: GetUserRequest) -> rupring::Result<GetUserResponse>;
    fn list_users(&self, request: ListUsersRequest) -> rupring::Result<ListUsersResponse>;
}

#[mockall::automock]
pub trait IUserRepository {
    fn create_user(&self, params: CreateUserParams) -> rupring::Result<i32>;
    fn update_user(&self, params: UpdateUserParams) -> rupring::Result<()>;
    fn delete_user(&self, params: DeleteUserParams) -> rupring::Result<()>;
    fn get_user(&self, params: GetUserParams) -> rupring::Result<Option<User>>;
    fn list_users(&self, params: ListUsersParams) -> rupring::Result<Users>;
    fn count_users(&self, params: CountUsersParams) -> rupring::Result<i32>;
}
