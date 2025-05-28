use anyhow::Result;

use crate::{domain::{admin::{Admin, NewAdmin, RegisterAdminRequest}, admin::{LoginCredentials}}, infrastructure::{admin_repository::AdminRepository}};

#[derive(Debug, Clone)]
pub struct AdminService {
    repo: AdminRepository,
}

impl AdminService {
    pub fn new(repo: AdminRepository) -> Self {
        AdminService { repo }
    }

    pub async fn register_admin(&self, request: RegisterAdminRequest) -> Result<Admin, String> {
        let hashed_password = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
            .map_err(|e| format!("Failed to hash password: {}", e))?;
        let new_admin = NewAdmin {
            username: &request.username,
            password_hash: &hashed_password,
        };

        self.repo.register_admin(new_admin).await
    }

    pub async fn login_admin(&self,credentials:LoginCredentials)->Result<i32,String>{
        let admin = self.repo.find_admin_by_username_and_verify_password(credentials).await?;

        Ok(admin.id)
    }
}
