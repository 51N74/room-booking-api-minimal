use anyhow::Result;
use diesel::SqliteConnection;

use crate::{
    domain::{
        admin::LoginCredentials,
        admin::{Admin, NewAdmin, RegisterAdminRequest},
    },
    infrastructure::admin_repository::AdminRepository,
};

#[derive(Debug, Clone)]
pub struct AdminService {
    repo: AdminRepository,
}

impl AdminService {
    pub fn new(repo: AdminRepository) -> Self {
        AdminService { repo }
    }

    pub async fn register_admin(&self, conn: &mut SqliteConnection,request: RegisterAdminRequest) -> Result<Admin, String> {
        let hashed_password = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
            .map_err(|e| format!("Failed to hash password: {}", e))?;
        let new_admin = NewAdmin {
            username: &request.username,
            password_hash: &hashed_password,
        };

        self.repo.register_admin(conn, new_admin).await // 
    }

    pub async fn login_admin(&self,_conn: &mut SqliteConnection, credentials: LoginCredentials) -> Result<i32, String> {
        let admin = self
            .repo
            .find_admin_by_username_and_verify_password(credentials)
            .await?;

        Ok(admin.id)
    }
}
