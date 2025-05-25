use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;


use crate::{application::room_service::RoomService, domain::room::RoomDTO};

#[derive(Clone,Deserialize)]
pub struct CreateRoomRequest{
    pub name:String,
    pub status:String,
}

pub async fn create_room_handler(
     State(state):State<RoomService>,
    Json(payload):Json<CreateRoomRequest>
)-> impl IntoResponse{
    let room_dto = RoomDTO{
        name:payload.name,
        status:payload.status
    };

      match state.create_room(room_dto).await{
       Ok(room) => (StatusCode::CREATED, Json(room)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}