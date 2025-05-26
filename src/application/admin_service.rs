use crate::{domain::admin::RegisterAdminEntity, infrastructure::admin_repository::AdminRepository};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct AdminService{
    repo:AdminRepository
}

impl AdminService{
    pub fn new(repo:AdminRepository)->Self{
        AdminService {repo}
    }

    pub async fn register_admin(&self,admin:RegisterAdminEntity)->Result<RegisterAdminEntity>{
        self.repo.register_admin(admin).await
    }
}