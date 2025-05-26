use crate::{domain::room::{RoomDTO, RoomEntity}, infrastructure::room_repository::RoomRepository};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct RoomService{
    repo:RoomRepository
}

impl RoomService{
    pub fn new(repo:RoomRepository)->Self{
        RoomService {repo}
    }

    pub async fn create_room(&self,room:RoomDTO)->Result<RoomEntity>{
        self.repo.create_room(room).await
    }

   
}