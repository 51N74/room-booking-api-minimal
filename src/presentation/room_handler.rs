use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::{application::{room_service::RoomService, user_service::UserService}, domain::{room::AddRoomEntity, user::RegisterUserEntity}};

#[derive(Clone, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub status: String,
}

pub async fn create_room_handler(
    State(state): State<RoomService>,
    Json(payload): Json<CreateRoomRequest>,
) -> impl IntoResponse {
    let room_dto = AddRoomEntity {
        id:0,
        name: payload.name,
        status: payload.status,
    };

    match state.create_room(room_dto).await {
        Ok(room) => (StatusCode::CREATED, Json(room)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_all_rooms_handler(
    State(state):State<RoomService>,
    )-> impl IntoResponse{
        match state.get_all_rooms().await{
            Ok(rooms)=> (StatusCode::OK, Json(rooms)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }


