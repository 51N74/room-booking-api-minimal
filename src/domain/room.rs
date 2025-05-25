use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::infrastructure::schema::rooms;

#[derive(Debug,Clone,Insertable,Queryable,Serialize,Deserialize)]
#[diesel(table_name = rooms)]
pub struct RoomEntity {
    pub id: i32,
    pub name: String,
    pub status: String, // "available" or "booked"
}

#[derive(Debug,Clone,Serialize,Deserialize)]pub struct RoomDTO {
    pub name: String,
    pub status: String,
}

