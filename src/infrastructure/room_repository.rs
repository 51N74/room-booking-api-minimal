use crate::domain::room::NewRoom;
use crate::domain::room::Room;
use crate::domain::room::RoomChangeset;

use chrono::Local;
use chrono::Utc;
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

    pub fn update_room_status_sync(
        conn: &mut SqliteConnection,
        room_id: i32,
        new_status: &str,
    ) -> Result<Room, diesel::result::Error> {
        
        let updated_rows = diesel::update(rooms::table.find(room_id))
            .set((
                rooms::status.eq(new_status),
                rooms::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?; 

        
        if updated_rows == 0 {
        
            return Err(diesel::result::Error::NotFound);
        }

        
        rooms::table
            .find(room_id)
            .select(Room::as_select())
            .first(conn) 
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
            .map_err(|e| format!("Failed to insert room into DB: {}", e))?;

        let inserted_room = rooms::table
            .filter(rooms::name.eq(&new_room_data.name))
            .first::<Room>(&mut conn)
            .map_err(|e| format!("Failed to retrieve newly inserted room: {}", e))?;

        Ok(inserted_room)
    }

    pub async fn get_all_room(&self) -> Result<Vec<Room>, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?;

        let rooms = rooms::table
            .load::<Room>(&mut conn)
            .map_err(|e| format!("Failed to retrieve all users: {}", e))?;
        Ok(rooms)
    }
    pub async fn get_room_by_id(&self, room_id: i32) -> Result<Room, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?; // <<-- ตรงนี้ดึง Connection จาก Pool

        let room = rooms::table // <<-- อ้างถึงตาราง rooms
            .filter(rooms::id.eq(room_id))
            .filter(rooms::deleted_at.is_null()) 
            .first::<Room>(&mut conn) // <<-- ดึงข้อมูลแรกที่เจอและแปลงเป็น Struct Room
            .map_err(|e| format!("Failed to retrieve room by ID: {}", e))?; // <<-- จัดการ Error ถ้าไม่พบหรือไม่สำเร็จ

        Ok(room) // <<-- คืนค่า Room ที่พบ
    }

    pub async fn get_all_active_rooms(&self)->Result<Vec<Room>,String>{
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?; // <<-- ตรงนี้ดึง Connection จาก Pool
        let rooms = rooms::table
            .filter(rooms::deleted_at.is_null()) 
            .filter(rooms::status.eq("available"))
            .load::<Room>(&mut conn)
            .map_err(|e| format!("Failed to retrieve all users: {}", e))?;
        Ok(rooms)
    }





    pub async fn update_room(&self, room_id: i32, changes: RoomChangeset) -> Result<Room, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?; // <<-- ตรงนี้ดึง Connection จาก Pool
        let update_rows = diesel::update(rooms::table.filter(rooms::id.eq(room_id)))
            .set(&changes)
            .execute(&mut conn)
            .map_err(|e| format!("Failed to retrieve room by ID: {}", e))?; // <<-- จัดการ Error ถ้าไม่พบหรือไม่สำเร็จ
        if update_rows == 0 {
            return Err("Room not found".to_string());
        }
        let updated_room = rooms::table
            .filter(rooms::id.eq(room_id))
            .first::<Room>(&mut conn)
            .map_err(|e| format!("Failed to retrieve room by ID: {}", e))?;

        Ok(updated_room)
    }
    pub async fn delete_room(&self, room_id: i32 ) -> Result<Room, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?; // <<-- ตรงนี้ดึง Connection จาก Pool
        
        let changes = RoomChangeset {
            name: None,          // ไม่ได้อัปเดตชื่อSome
            status: None,   // ไม่ได้อัปเดตคำอธิบาย
            updated_at: Some(Local::now().naive_local()), // อัปเดต updated_at ด้วย
            deleted_at: Some(Local::now().naive_local()), // <<-- ตั้งค่า deleted_at
        };
        
        let updated_rows = diesel::update(rooms::table.filter(rooms::id.eq(room_id)))
            .set(changes)
            .execute(&mut conn)
            .map_err(|e| format!("Failed to soft delete room: {}", e))?;

        if updated_rows == 0 {
            return Err("Room not found or already deleted".to_string());
        } 
        
        let updated_room = rooms::table
            .filter(rooms::id.eq(room_id))
            .first::<Room>(&mut conn)
            .map_err(|e| format!("Failed to retrieve room by ID: {}", e))?;
        
        Ok(updated_room)

    }
}
