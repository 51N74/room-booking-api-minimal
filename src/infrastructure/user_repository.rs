use crate::domain::user::LoginCredentials;
use crate::domain::user::NewUser;
use crate::domain::user::User;

use super::schema::users;

use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl UserRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        UserRepository { pool }
    }

    pub async fn register_user(&self, new_user_data: NewUser<'_>) -> Result<User, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?;

        // ตรวจสอบว่า username ซ้ำหรือไม่ (เป็น Good Practice)
        let existing_user_result = users::table
            .filter(users::username.eq(&new_user_data.username))
            .first::<User>(&mut conn);

        match existing_user_result {
            Ok(_) => {
                // ถ้า User ถูกพบ แสดงว่า username ซ้ำ
                return Err("Username already taken".to_string());
            }
            Err(diesel::result::Error::NotFound) => {
                // ถ้าไม่พบผู้ใช้ ถือว่าถูกต้อง สามารถดำเนินการต่อได้
                // ไม่ต้องทำอะไรตรงนี้
            }
            Err(e) => {
                // จัดการกับ Error อื่นๆ ที่ไม่ใช่ NotFound
                return Err(format!("Database error during user check: {}", e));
            }
        }

        diesel::insert_into(users::table)
            .values(&new_user_data)
            .execute(&mut conn)
            .map_err(|e| format!("Failed to insert user into DB: {}", e))?;

        let inserted_user = users::table
            .filter(users::username.eq(&new_user_data.username))
            .first::<User>(&mut conn)
            .map_err(|e| format!("Failed to retrieve newly inserted user: {}", e))?;

        Ok(inserted_user)

    }
    // ฟังก์ชันสำหรับค้นหาผู้ใช้ด้วย Username และตรวจสอบรหัสผ่าน›
    pub async fn find_user_by_username_and_verify_password(
        &self,
        login_credentials: LoginCredentials,
    ) -> Result<User, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?;

        let user = users::table
            .filter(users::username.eq(&login_credentials.username))
            .first::<User>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => "Invalid username or password".to_string(),
                _ => format!("Database error while fetching user: {}", e),
            })?;

        // ตรวจสอบรหัสผ่าน (Plain Text) กับ password_hash ที่เก็บใน DB
        let password_matches = bcrypt::verify(&login_credentials.password, &user.password_hash)
            .map_err(|e| format!("Password verification internal error: {}", e))?;

        if password_matches {
            Ok(user)
        } else {
            Err("Invalid username or password".to_string())
        }
    }
}
