use anyhow::Result;
use diesel::SqliteConnection;
use diesel::prelude::*;
use crate::domain::room::{RoomDTO, RoomEntity};

use super::schema::rooms;

#[derive(Debug, Clone)]
pub struct RoomRepository{
    pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>,
}

impl RoomRepository{
    pub fn new(database_url:&str)->Self{
         let manager = diesel::r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = diesel::r2d2::Pool::builder().build(manager).expect("Failed to create pool");
        RoomRepository { pool }
    }

    pub async fn create_room(&self,room:RoomDTO)->Result<RoomEntity>{
        let new_room = (
            rooms::name.eq(room.name.clone()),
            rooms::status.eq(room.status.clone())
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
            name:room.name,
            status: room.status,
        })
    }



   pub async fn get_all_rooms(&self) -> Result<Vec<RoomEntity>> {
    
        // Clone the pool to move it into the blocking task
        let pool = self.pool.clone();

        // ย้ายการทำงานที่เป็น Blocking I/O ไปอยู่ใน spawn_blocking
        let results = tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().unwrap(); // ดึง connection ภายใน blocking task
            // ใช้ rooms (ที่มาจาก dsl) และเรียก load ด้วย Generic Type RoomEntity
            rooms::table.load::<RoomEntity>(&mut conn)
        })
        .await??; // จัดการ JoinError และ Error จาก Diesel/r2d2

        Ok(results)
    }
}

