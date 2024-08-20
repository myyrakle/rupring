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
    fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<CreateUserResponse>;
    fn update_user(&self, request: UpdateUserRequest) -> anyhow::Result<UpdateUserResponse>;
    fn delete_user(&self, request: DeleteUserRequest) -> anyhow::Result<DeleteUserResponse>;
    fn get_user(&self, request: GetUserRequest) -> anyhow::Result<GetUserResponse>;
    fn list_users(&self, request: ListUsersRequest) -> anyhow::Result<ListUsersResponse>;
}

pub trait IUserRepository {
    fn create_user(&self, params: CreateUserParams) -> anyhow::Result<i32>;
    fn update_user(&self, params: UpdateUserParams) -> anyhow::Result<()>;
    fn delete_user(&self, params: DeleteUserParams) -> anyhow::Result<()>;
    fn get_user(&self, params: GetUserParams) -> anyhow::Result<Option<User>>;
    fn list_users(&self, params: ListUsersParams) -> anyhow::Result<Users>;
    fn count_users(&self, params: CountUsersParams) -> anyhow::Result<i32>;
}
