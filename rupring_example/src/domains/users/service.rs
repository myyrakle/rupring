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

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::domains::users::interface::MockIUserRepository;

    #[test]
    fn test_create_user() {
        struct TestCase {
            name: String,
            request: CreateUserRequest,
            expected: CreateUserResponse,
            want_error: bool,
            mock_user_repository: fn() -> Arc<dyn IUserRepository>,
        }

        let test_cases = vec![
            TestCase {
                name: "create user success".to_string(),
                request: CreateUserRequest {
                    username: "username".to_string(),
                    email: "email".to_string(),
                    password: "password".to_string(),
                },
                expected: CreateUserResponse {},
                want_error: false,
                mock_user_repository: || {
                    let mut repository = MockIUserRepository::new();

                    repository
                        .expect_create_user()
                        .times(1)
                        .with(eq(CreateUserParams {
                            name: "username".to_string(),
                            email: "email".to_string(),
                            password: "password".to_string(),
                        }))
                        .returning(|_| Ok(1));

                    Arc::new(repository)
                },
            },
            TestCase {
                name: "create user failed".to_string(),
                request: CreateUserRequest {
                    username: "username".to_string(),
                    email: "email".to_string(),
                    password: "password".to_string(),
                },
                expected: CreateUserResponse {},
                want_error: true,
                mock_user_repository: || {
                    let mut repository = MockIUserRepository::new();

                    repository
                        .expect_create_user()
                        .times(1)
                        .with(eq(CreateUserParams {
                            name: "username".to_string(),
                            email: "email".to_string(),
                            password: "password".to_string(),
                        }))
                        .returning(|_| Err(rupring::error!("error")));

                    Arc::new(repository)
                },
            },
        ];

        for t in test_cases {
            let service = UserService::new((t.mock_user_repository)());

            let got = service.create_user(t.request);

            assert_eq!(
                got.is_err(),
                t.want_error,
                "{}: want_error: {}, error: {:?}",
                t.name,
                t.want_error,
                got.err()
            );

            if let Ok(tokens) = got {
                assert_eq!(tokens, t.expected, "{}", t.name);
            }
        }
    }
}

#[rupring::Injectable]
fn inject_user_service(repository: Arc<dyn IUserRepository>) -> Arc<dyn IUserService> {
    Arc::new(UserService::new(repository))
}
