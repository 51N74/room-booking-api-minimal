use anyhow::Result;

use crate::{domain::user::{LoginCredentials, NewUser, RegisterUserRequest, User}, infrastructure::user_repository::UserRepository};
use bcrypt;
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

    // *** ฟังก์ชันใหม่สำหรับ Admin Management (Regular Users) ***

    // ฟังก์ชันดึง User ทั้งหมด (เรียกจาก UserRepository)
    pub async fn get_all_users(&self) -> Result<Vec<User>, String> { // Result<Vec<User>, String>
        self.repo.get_all_users().await // เรียก UserRepository ซึ่งเป็น async
    }

    pub async fn get_user_by_id(&self,user_id:i32)->Result<User,String>{
        self.repo.get_user_by_id(user_id).await
    }

    // ฟังก์ชัน Soft Delete User (เรียกจาก UserRepository)
    pub async fn delete_user(&self, user_id: i32) -> Result<(), String> { // Result<(), String>
        let success = self.repo.soft_delete_user(user_id).await?; // เรียก UserRepository ซึ่งเป็น async
        if !success {
            return Err("User not found or already deleted".to_string()); // สามารถปรับข้อความ Error ให้ตรงกับ UserServiceError ได้
        }
        Ok(())
    }
}
