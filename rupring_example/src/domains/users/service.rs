use std::sync::Arc;

use super::{
    dto::{
        CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse,
        GetUserRequest, GetUserResponse, ListUsersRequest, ListUsersResponse, UpdateUserRequest,
        UpdateUserResponse,
    },
    interface::{IUserRepository, IUserService},
    model::{CreateUserParams, GetUserParams, ListUsersParams},
};

pub struct UserService {
    repository: Arc<dyn IUserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn IUserRepository>) -> Self {
        Self { repository }
    }
}

impl IUserService for UserService {
    fn create_user(&self, request: CreateUserRequest) -> rupring::Result<CreateUserResponse> {
        let _ = self.repository.create_user(CreateUserParams {
            name: request.username,
            email: request.email,
            password: request.password,
        })?;

        Ok(CreateUserResponse {})
    }

    fn update_user(&self, request: UpdateUserRequest) -> rupring::Result<UpdateUserResponse> {
        let _ = self
            .repository
            .update_user(super::model::UpdateUserParams {
                id: request.id,
                name: request.username,
                email: request.email,
                password: request.password,
            })?;

        Ok(UpdateUserResponse {})
    }

    fn delete_user(&self, request: DeleteUserRequest) -> rupring::Result<DeleteUserResponse> {
        self.repository
            .delete_user(super::model::DeleteUserParams { id: request.id })?;

        Ok(DeleteUserResponse {})
    }

    fn get_user(&self, request: GetUserRequest) -> rupring::Result<GetUserResponse> {
        let user = self.repository.get_user(GetUserParams { id: request.id })?;

        if let Some(user) = user {
            return Ok(GetUserResponse {
                id: user.id,
                username: user.name,
                email: user.email,
            });
        }

        Err(rupring::error!("user not found"))
    }

    fn list_users(&self, request: ListUsersRequest) -> rupring::Result<ListUsersResponse> {
        let count = self
            .repository
            .count_users(super::model::CountUsersParams {})?;

        let users = self.repository.list_users(ListUsersParams {
            limit: request.limit,
            offset: request.offset,
        })?;

        Ok(ListUsersResponse {
            total: count,
            users: users
                .into_iter()
                .map(|user| GetUserResponse {
                    id: user.id,
                    username: user.name,
                    email: user.email,
                })
                .collect(),
        })
    }
}

#[rupring::Injectable]
fn inject_user_service(repository: Arc<dyn IUserRepository>) -> Arc<dyn IUserService> {
    Arc::new(UserService::new(repository))
}
