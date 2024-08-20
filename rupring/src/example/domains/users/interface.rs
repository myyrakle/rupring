use super::dto::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, GetUserRequest,
    GetUserResponse, ListUsersRequest, ListUsersResponse, UpdateUserRequest, UpdateUserResponse,
};

pub trait IUserService {
    fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<CreateUserResponse>;
    fn update_user(&self, request: UpdateUserRequest) -> anyhow::Result<UpdateUserResponse>;
    fn delete_user(&self, request: DeleteUserRequest) -> anyhow::Result<DeleteUserResponse>;
    fn get_user(&self, request: GetUserRequest) -> anyhow::Result<GetUserResponse>;
    fn list_users(&self, request: ListUsersRequest) -> anyhow::Result<ListUsersResponse>;
}

pub trait IUserRepository {}
