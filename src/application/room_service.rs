use anyhow::Result;
use chrono::Utc;

use crate::{
    domain::room::{AddRoomRequest, NewRoom, Room, RoomChangeset, UpdateRoomRequest},
    infrastructure::room_repository::RoomRepository,
};

#[derive(Debug, Clone)]
pub struct RoomService {
    repo: RoomRepository,
}

impl RoomService {
    pub fn new(repo: RoomRepository) -> Self {
        RoomService { repo }
    }

    pub async fn add_room(&self, request: AddRoomRequest) -> Result<Room, String> {
        let new_room = NewRoom {
            name: &request.name,
            status: &request.status,
        };

        self.repo.add_room(new_room).await
    }

    pub async fn get_all_room(&self)->Result<Vec<Room>,String>{
        self.repo.get_all_room().await
    }  
    pub async fn get_room_by_id(&self,room_id:i32)->Result<Room,String>{
        self.repo.get_room_by_id(room_id).await
    }


    pub async fn get_all_active_rooms(&self) -> Result<Vec<Room>, String> {
        self.repo.get_all_active_rooms().await
    }

    pub async fn update_room(&self,room_id:i32,request:UpdateRoomRequest)->Result<Room,String>{
        let changes = RoomChangeset{
            name:request.name,
            status:request.status,
            updated_at:Some(Utc::now().naive_utc()),
            deleted_at:None
        };
        self.repo.update_room(room_id,changes).await
    }

    pub async fn delete_room(&self,room_id:i32)->Result<Room,String>{
        
        self.repo.delete_room(room_id).await
    }
    
}
