use anyhow::Result;

use crate::{
    domain::room::{AddRoomRequest, NewRoom, Room},
    infrastructure::{room_repository::RoomRepository},
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
}
