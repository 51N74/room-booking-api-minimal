use chrono::NaiveDateTime;
use diesel::{prelude::AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::infrastructure::schema::rooms;
// Room: Entity ที่แทนข้อมูลผู้ใช้ในฐานข้อมูล (เมื่อดึงออกมาหรือบันทึกเสร็จแล้ว)
// มี Field ครบทุกคอลัมน์ในตาราง users
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub status: String, 
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
    pub deleted_at: Option<NaiveDateTime>,
}

// AddURoomRequest: Struct ที่ใช้รับข้อมูลจาก Client สำหรับการลงทะเบียน
// มี สถานะไม่มี id เพราะ DB จะสร้างให้
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRoomRequest {
    pub name: String,
    pub status: String, // <<-- สถานะ จาก client
}

// NewRoom: Struct ที่ใช้สำหรับการ INSERT ข้อมูลใหม่ลงในฐานข้อมูล
// มีเฉพาะ Field ที่เราต้องการระบุค่าตอน INSERT
#[derive(Debug, Insertable)]
#[diesel(table_name = rooms)]
pub struct NewRoom<'a> {
    pub name: &'a str,
    pub status: &'a str, // <<-- สถานะห้อง
}


// RoomChangeset: Struct สำหรับส่งข้อมูลไป UPDATE
// มีเฉพาะ Field ที่ต้องการให้ Update ได้
#[derive(Debug, Clone, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = rooms)] // <<-- ต้องระบุ table_name
pub struct RoomChangeset {
    pub name: Option<String>,        // ใช้ Option เพื่อรองรับ Partial Update
    pub status:Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>, // ถ้าต้องการ update deleted_at
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoomRequest{
    pub name:Option<String>,
    pub status:Option<String>
}