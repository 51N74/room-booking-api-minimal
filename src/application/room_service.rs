use crate::{domain::room::{AddRoomEntity, RoomEntity}, infrastructure::room_repository::RoomRepository};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct RoomService{
    repo:RoomRepository
}

impl RoomService{
    pub fn new(repo:RoomRepository)->Self{
        RoomService {repo}
    }

    pub async fn create_room(&self,room:AddRoomEntity)->Result<RoomEntity>{
        self.repo.create_room(room).await
    }

    pub async fn get_all_rooms(&self)->Result<Vec<RoomEntity>>{
        self.repo.get_all_rooms().await
    }

    pub async fn get_room_by_id(&self,room_id:i32)->Result<RoomEntity>{
        self.repo.get_room_by_id(room_id).await
    }

    pub async fn get_room_by_status(&self,room_status:String)->Result<Vec<RoomEntity>>{
        self.repo.get_room_by_status(room_status).await
    }

    pub async fn update_room(&self,room:RoomEntity)->Result<RoomEntity>{
        self.repo.update_room(room).await
    }
   
   pub async fn delete_room(&self,room_id:i32)->Result<RoomEntity>{
        self.repo.delete_room(room_id).await
}


}