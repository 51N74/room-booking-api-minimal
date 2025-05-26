use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{application::{room_service::RoomService, user_service::UserService}, domain::{room::{AddRoomEntity, RoomEntity}, user::RegisterUserEntity}};

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

pub async fn get_room_by_id_handler(
    State(state): State<RoomService>, // อันนี้ถูกต้องแล้ว
    Path(room_id): Path<i32>, // <-- แก้ไขตรงนี้!
) -> impl IntoResponse {
    match state.get_room_by_id(room_id).await {
        Ok(room) => (StatusCode::OK, Json(room)).into_response(),
        Err(e) => {
            eprintln!("Error getting room by id: {}", e); // เพิ่ม log error สำหรับ debug
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        },
    }
}


pub async fn update_room_handler(
    State(state): State<RoomService>,
    Json(payload): Json<RoomEntity>,
) -> impl IntoResponse {
    match state.update_room(payload).await {
        Ok(room) => (StatusCode::OK, Json(room)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
pub async fn delete_room_handler(
    State(state): State<RoomService>,
    Path(room_id): Path<i32>,
) -> impl IntoResponse {
    match state.delete_room(room_id).await {
        Ok(room) => {
            println!("Room deleted successfully: {:?}", room); // เพิ่ม log สำหรับการลบห้อง
            (StatusCode::OK, Json(room),).into_response()},
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}


// Struct สำหรับรับ Query Parameter
#[derive(Debug, Deserialize)]
pub struct GetRoomsByStatusQuery {
    pub status: String, // <-- เปลี่ยนตรงนี้
}
pub async fn get_rooms_by_status_handler(
    State(state): State<RoomService>,
    Query(query): Query<GetRoomsByStatusQuery>,
) -> impl IntoResponse {
    // ไม่ต้องตรวจสอบ query.status.is_empty() อีกต่อไป เพราะมันเป็น String แล้ว
    if query.status.is_empty() { // อาจจะเช็คได้ ถ้าคุณอยากให้ string ว่างเปล่าเป็น BadRequest
         return (StatusCode::BAD_REQUEST, "Status parameter cannot be empty").into_response();
    }

    // เรียกใช้ฟังก์ชันที่รับ String เดียว (ถ้าคุณมี)
    match state.get_room_by_status(query.status).await {
        Ok(rooms) => (StatusCode::OK, Json(rooms)).into_response(),
        Err(e) => {
            eprintln!("Error fetching room by status: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve room").into_response()
        }
    }
}