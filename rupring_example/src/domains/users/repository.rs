use std::sync::{Arc, Mutex};

use super::{interface::IUserRepository, model::Users};

pub struct UserInMemoryRepository {
    users: Arc<Mutex<Users>>,
}

impl UserInMemoryRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Users::new())),
        }
    }
}

impl IUserRepository for UserInMemoryRepository {
    fn create_user(&self, params: super::model::CreateUserParams) -> rupring::Result<i32> {
        if self.users.is_poisoned() {
            self.users.clear_poison();
        }

        let mut users = self.users.lock().unwrap();

        let id = users.len() as i32 + 1;

        let user = super::model::User {
            id,
            name: params.name,
            email: params.email,
            password: params.password,
        };

        users.push(user);

        Ok(id)
    }

    fn update_user(&self, params: super::model::UpdateUserParams) -> rupring::Result<()> {
        if self.users.is_poisoned() {
            self.users.clear_poison();
        }

        let mut users = self.users.lock().unwrap();

        let user = users.iter_mut().find(|user| user.id == params.id);
        if let Some(user) = user {
            user.name = params.name.clone();
            user.email = params.email.clone();
            user.password = params.password.clone();
        }

        Ok(())
    }

    fn delete_user(&self, params: super::model::DeleteUserParams) -> rupring::Result<()> {
        if self.users.is_poisoned() {
            self.users.clear_poison();
        }

        let mut users = self.users.lock().unwrap();
        let new_list = users
            .iter()
            .filter(|user| user.id != params.id)
            .cloned()
            .collect();

        *users = new_list;

        Ok(())
    }

    fn get_user(
        &self,
        request: super::model::GetUserParams,
    ) -> rupring::Result<Option<super::model::User>> {
        if self.users.is_poisoned() {
            self.users.clear_poison();
        }

        let users = self.users.lock().unwrap();

        let user = users.iter().find(|user| user.id == request.id);

        Ok(user.cloned())
    }

    fn list_users(&self, params: super::model::ListUsersParams) -> rupring::Result<Users> {
        if self.users.is_poisoned() {
            self.users.clear_poison();
        }

        let users = self.users.lock().unwrap();

        let mut start = params.offset as usize;
        let mut end = start + params.limit as usize;

        if users.is_empty() {
            return Ok(vec![]);
        }

        if start >= users.len() {
            start = users.len() - 1;
        }

        if end >= users.len() {
            end = users.len() - 1;
        }

        Ok(users[start..end].to_vec())
    }

    fn count_users(&self, _: super::model::CountUsersParams) -> rupring::Result<i32> {
        if self.users.is_poisoned() {
            self.users.clear_poison();
        }

        let users = self.users.lock().unwrap();

        Ok(users.len() as i32)
    }
}

#[rupring::Injectable]
fn inject_user_repository() -> Arc<dyn IUserRepository> {
    Arc::new(UserInMemoryRepository::new())
}
