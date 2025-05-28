// src/presentation/user_handler.rs

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize};

use crate::application::room_service::RoomService;




// Request Body สำหรับการลงทะเบียน (รับรหัสผ่านดิบจาก Client)
#[derive(Clone, Deserialize)]
pub struct AddRoomRequest {
    pub name: String,
    pub status: String, // <<-- รหัสผ่านดิบ (Plain Text)
}

// Handler สำหรับการลงทะเบียนผู้ใช้ (POST /register)
pub async fn add_room_handler(
    State(room_service): State<RoomService>,
    Json(payload): Json<AddRoomRequest>, // รับ CreateUserRequest
) -> impl IntoResponse {
    // แปลง CreateUserRequest ไปเป็น RegisterUserRequest
    // (ตอนนี้ CreateUserRequest และ RegisterUserRequest มีโครงสร้างคล้ายกันมาก อาจจะใช้ตัวเดียวกันก็ได้ แต่แยกไว้ชัดเจนกว่า)
    let add_room_request = crate::domain::room::AddRoomRequest {
        name: payload.name,
        status: payload.status,
    };

    match room_service.add_room(add_room_request).await {
        Ok(room) => (StatusCode::CREATED, Json(room)).into_response(), // UserEntity (User) derive Serialize
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(), // <<-- เปลี่ยนเป็น BAD_REQUEST สำหรับ Error ทั่วไป เช่น username ซ้ำ
    }
}

