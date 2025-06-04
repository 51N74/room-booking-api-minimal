use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::infrastructure::schema::admins;

// Admin: Entity ที่แทนข้อมูลผู้ใช้ในฐานข้อมูล (เมื่อดึงออกมาหรือบันทึกเสร็จแล้ว)
// มี Field ครบทุกคอลัมน์ในตาราง users
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
#[diesel(table_name = admins)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub password_hash: String, // <<-- รหัสผ่านที่ถูก Hash แล้ว
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, // <<-- ถ้ามี updated_at ใน DB
    pub deleted_at: Option<NaiveDateTime>
}

// RegisterAdminRequest: Struct ที่ใช้รับข้อมูลจาก Client สำหรับการลงทะเบียน
// มีรหัสผ่านดิบ (Plain Text) และไม่มี id เพราะ DB จะสร้างให้
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAdminRequest {
    pub username: String,
    pub password: String, // <<-- รหัสผ่านดิบ (Plain Text) จาก Client
}

// NewAdmin: Struct ที่ใช้สำหรับการ INSERT ข้อมูลใหม่ลงในฐานข้อมูล
// มีเฉพาะ Field ที่เราต้องการระบุค่าตอน INSERT
#[derive(Debug, Insertable)]
#[diesel(table_name = admins)]
pub struct NewAdmin<'a> {
    pub username: &'a str,
    pub password_hash: &'a str, // <<-- รหัสผ่านที่ถูก Hash แล้ว
}

// LoginCredentials: Struct ที่ใช้รับข้อมูลจาก Client สำหรับการ Login
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String, // <<-- รหัสผ่านดิบ (Plain Text) จาก Client
}
