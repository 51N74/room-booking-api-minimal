use anyhow::Result;

use crate::{domain::user::{LoginCredentials, NewUser, RegisterUserRequest, User}, infrastructure::user_repository::UserRepository};

#[derive(Debug, Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        UserService { repo }
    }

    pub async fn register_user(&self, request: RegisterUserRequest) -> Result<User, String> {
        let hashed_password = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
            .map_err(|e| format!("Failed to hash password: {}", e))?;
        let new_user = NewUser {
            username: &request.username,
            password_hash: &hashed_password,
        };

        self.repo.register_user(new_user).await
    }

    pub async fn login_user(&self,credentials:LoginCredentials)->Result<i32,String>{
        let user = self.repo.find_user_by_username_and_verify_password(credentials).await?;

        Ok(user.id)
    }
}
