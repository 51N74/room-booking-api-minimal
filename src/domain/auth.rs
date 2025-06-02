use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user_id: i32,
    pub token: String,      // เพิ่ม token ใน response
    pub role: String,       // เพิ่ม role ใน response
    pub expires_in: i64,    // เวลาหมดอายุ (วินาที)
}