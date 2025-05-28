use crate::domain::room::NewRoom;
use crate::domain::room::Room;

use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use super::schema::rooms;

#[derive(Debug, Clone)]
pub struct RoomRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl RoomRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        RoomRepository { pool }
    }

    pub async fn add_room(&self, new_room_data: NewRoom<'_>) -> Result<Room, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?;

        // ตรวจสอบว่า name ซ้ำหรือไม่ (เป็น Good Practice)
        let existing_room_result = rooms::table
            .filter(rooms::name.eq(&new_room_data.name))
            .first::<Room>(&mut conn);

        match existing_room_result {
            Ok(_) => {
                // ถ้า User ถูกพบ แสดงว่า username ซ้ำ
                return Err("Room name already taken".to_string());
            }
            Err(diesel::result::Error::NotFound) => {
                // ถ้าไม่พบผู้ใช้ ถือว่าถูกต้อง สามารถดำเนินการต่อได้
                // ไม่ต้องทำอะไรตรงนี้
            }
            Err(e) => {
                // จัดการกับ Error อื่นๆ ที่ไม่ใช่ NotFound
                return Err(format!("Database error during room check: {}", e));
            }
        }

        diesel::insert_into(rooms::table)
            .values(&new_room_data)
            .execute(&mut conn)
            .map_err(|e| format!("Failed to insert user into DB: {}", e));

        let inserted_room = rooms::table
            .filter(rooms::name.eq(&new_room_data.name))
            .first::<Room>(&mut conn)
            .map_err(|e| format!("Failed to retrieve newly inserted user: {}", e))?;

        Ok(inserted_room)
    }
}