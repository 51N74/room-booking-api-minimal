use crate::domain::room::{AddRoomEntity, RoomEntity};
use anyhow::Ok;
use anyhow::Result;
use diesel::SqliteConnection;
use diesel::prelude::*;

use super::schema::rooms;

#[derive(Debug, Clone)]
pub struct RoomRepository {
    pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>,
}

impl RoomRepository {
    pub fn new(database_url: &str) -> Self {
        let manager = diesel::r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = diesel::r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool");
        RoomRepository { pool }
    }

    pub async fn create_room(&self, room: AddRoomEntity) -> Result<RoomEntity> {
        let new_room = (
            rooms::name.eq(room.name.clone()),
            rooms::status.eq(room.status.clone()),
        );

        let mut conn = self.pool.get()?;

        diesel::insert_into(rooms::table)
            .values(&new_room)
            .execute(&mut conn)
            .map_err(|e| anyhow::anyhow!("Failed to insert order: {}", e))?;

        let id = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result::<i32>(&mut conn)
        .map_err(|e| anyhow::anyhow!("Failed to get last insert id: {}", e))?;

        Ok(RoomEntity {
            id,
            name: room.name,
            status: room.status,
        })
    }

    //Get all rooms
    pub async fn get_all_rooms(&self) -> Result<Vec<RoomEntity>> {
        let pool = self.pool.clone();
        let mut conn = pool.get()?;

        let results = rooms::table
            .load::<RoomEntity>(&mut conn)
            .map_err(|e| anyhow::anyhow!("Failed to load rooms: {}", e))?;

        Ok(results)
    }

    //Get Rom id

    pub async fn get_room_by_id(&self, room_id: i32) -> Result<RoomEntity> {
        let pool = self.pool.clone();
        let mut conn = pool.get()?;

        let room = rooms::table
            .filter(rooms::id.eq(room_id))
            .first::<RoomEntity>(&mut conn)
            .map_err(|e| anyhow::anyhow!("Failed to find room: {}", e))?;

        Ok(room)
    }


    pub async fn get_room_by_status(&self, room_status: String) -> Result<Vec<RoomEntity>> { // <-- เปลี่ยนให้รับ String เดียว
    let pool = self.pool.clone();
    let mut conn = pool.get()?;

    let rooms_found = rooms::table
        .filter(rooms::status.eq(room_status)) // <-- ใช้ String ที่รับเข้ามาตรงๆ
        .load::<RoomEntity>(&mut conn)
        .map_err(|e| anyhow::anyhow!("Failed to load rooms by status: {}", e))?;

    Ok(rooms_found)
}



    //update Room

    pub async fn update_room(&self, room: RoomEntity) -> Result<RoomEntity> {
        let pool = self.pool.clone();
        let mut conn = pool.get()?;

        diesel::update(rooms::table.filter(rooms::id.eq(room.id)))
            .set((
                rooms::name.eq(room.name.clone()),
                rooms::status.eq(room.status.clone()),
            ))
            .execute(&mut conn)
            .map_err(|e| anyhow::anyhow!("Failed to update room: {}", e))?;

        Ok(room)
    }

    //delete

    pub async fn delete_room(&self, room_id: i32) -> Result<RoomEntity> {
        let pool = self.pool.clone();
        let mut conn = pool.get()?;

        let deleted_room = rooms::table
            .filter(rooms::id.eq(room_id))
            .first::<RoomEntity>(&mut conn)
            .optional()?
            .ok_or_else(|| anyhow::anyhow!("Room with id {} not found", room_id))?;
        Ok(deleted_room)
    }
}
