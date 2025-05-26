use crate::{domain::user::RegisterUserEntity, infrastructure::user_repository::UserRepository};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct UserService{
    repo:UserRepository
}

impl UserService{
    pub fn new(repo:UserRepository)->Self{
        UserService { repo  }
    }

    pub async fn register_user(&self,user:RegisterUserEntity)->Result<RegisterUserEntity>{
        self.repo.register_user(user).await
    }
}