// src/presentation/user_handler.rs

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{app_state::AppState};

// Request Body สำหรับการลงทะเบียน (รับรหัสผ่านดิบจาก Client)
#[derive(Clone, Deserialize)]
pub struct AddRoomRequest {
    pub name: String,
    pub status: String, // <<-- รหัสผ่านดิบ (Plain Text)
}

// Handler สำหรับการลงทะเบียนผู้ใช้ (POST /register)
pub async fn add_room_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<AddRoomRequest>, // รับ CreateUserRequest
) -> impl IntoResponse {
    // แปลง CreateUserRequest ไปเป็น RegisterUserRequest
    // (ตอนนี้ CreateUserRequest และ RegisterUserRequest มีโครงสร้างคล้ายกันมาก อาจจะใช้ตัวเดียวกันก็ได้ แต่แยกไว้ชัดเจนกว่า)
    let add_room_request = crate::domain::room::AddRoomRequest {
        name: payload.name,
        status: payload.status,
    };

    match app_state.room_service.add_room(add_room_request).await {
        Ok(room) => (StatusCode::CREATED, Json(room)).into_response(), // UserEntity (User) derive Serialize
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(), // <<-- เปลี่ยนเป็น BAD_REQUEST สำหรับ Error ทั่วไป เช่น username ซ้ำ
    }
}

pub async fn get_all_room_handler(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<crate::domain::room::Room>>, String> {
    app_state.room_service.get_all_room().await.map(Json)
}

// get room by id
pub async fn get_room_by_id_handler(
    State(app_state): State<AppState>,
    Path(room_id): Path<i32>,
) -> Result<Json<crate::domain::room::Room>, String> {
    app_state.room_service.get_room_by_id(room_id).await.map(Json)
}


pub async fn get_all_active_rooms_handler(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    match app_state.room_service.get_all_active_rooms().await {
        Ok(rooms) => (StatusCode::OK, Json(rooms)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn update_room_handler(
    State(app_state): State<AppState>,
    Path(room_id):Path<i32>,
    Json(payload):Json<crate::domain::room::UpdateRoomRequest>,
) -> impl IntoResponse{
    match app_state.room_service.update_room(room_id,payload).await{
        Ok(room) => (StatusCode::CREATED,Json(room)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST,e).into_response(),
    }
}

pub async fn delete_room_handler(
    State(app_state): State<AppState>,
    Path(room_id):Path<i32>,
) -> impl IntoResponse{
    match app_state.room_service.delete_room(room_id).await{
        Ok(room) => (StatusCode::CREATED,Json(room)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST,e).into_response(),
    }
}