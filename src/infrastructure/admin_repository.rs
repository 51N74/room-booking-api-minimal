use super::schema::admins;
use crate::domain::admin::Admin;
use crate::domain::admin::LoginCredentials;
use crate::domain::admin::NewAdmin;

use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

#[derive(Debug, Clone)]
pub struct AdminRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl AdminRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        AdminRepository { pool }
    }

    pub async fn register_admin(&self, new_admin_data: NewAdmin<'_>) -> Result<Admin, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?;

        // ตรวจสอบว่า username ซ้ำหรือไม่ (เป็น Good Practice)
        let existing_admin_result = admins::table
            .filter(admins::username.eq(&new_admin_data.username))
            .first::<Admin>(&mut conn);

        match existing_admin_result {
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
                return Err(format!("Database error during admin check: {}", e));
            }
        }

        diesel::insert_into(admins::table)
            .values(&new_admin_data)
            .execute(&mut conn)
            .map_err(|e| format!("Failed to insert user into DB: {}", e))?;

        let inserted_admin = admins::table
            .filter(admins::username.eq(&new_admin_data.username))
            .first::<Admin>(&mut conn)
            .map_err(|e| format!("Failed to retrieve newly inserted user: {}", e))?;

        Ok(inserted_admin)
    }
    // ฟังก์ชันสำหรับค้นหาผู้ใช้ด้วย Username และตรวจสอบรหัสผ่าน›
    pub async fn find_admin_by_username_and_verify_password(
        &self,
        login_credentials: LoginCredentials,
    ) -> Result<Admin, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?;

        let admin = admins::table
            .filter(admins::username.eq(&login_credentials.username))
            .first::<Admin>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => "Invalid username or password".to_string(),
                _ => format!("Database error while fetching user: {}", e),
            })?;

        // ตรวจสอบรหัสผ่าน (Plain Text) กับ password_hash ที่เก็บใน DB
        let password_matches = bcrypt::verify(&login_credentials.password, &admin.password_hash)
            .map_err(|e| format!("Password verification internal error: {}", e))?;

        if password_matches {
            Ok(admin)
        } else {
            Err("Invalid username or password".to_string())
        }
    }
}
