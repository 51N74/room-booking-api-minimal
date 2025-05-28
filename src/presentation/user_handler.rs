// src/presentation/user_handler.rs

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};


use crate::application::user_service::UserService;
// นำเข้า Struct จาก Domain Layer
use crate::domain::user::{User, LoginCredentials};

// Request Body สำหรับการลงทะเบียน (รับรหัสผ่านดิบจาก Client)
#[derive(Clone, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String, // <<-- รหัสผ่านดิบ (Plain Text)
}

// Request Body สำหรับการ Login (รับรหัสผ่านดิบจาก Client)
#[derive(Clone, Deserialize)]
pub struct LoginUserRequest {
    pub username: String,
    pub password: String, // <<-- รหัสผ่านดิบ (Plain Text)
}

// Response Body สำหรับการ Login (ถ้าต้องการส่ง Token หรือข้อมูลเพิ่มเติม)
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: i32,
    // pub token: String, // ถ้ามี Token-based authentication
}

// Handler สำหรับการลงทะเบียนผู้ใช้ (POST /register)
pub async fn register_user_handler(
    State(user_service): State<UserService>,
    Json(payload): Json<CreateUserRequest>, // รับ CreateUserRequest
) -> impl IntoResponse {
    // แปลง CreateUserRequest ไปเป็น RegisterUserRequest
    // (ตอนนี้ CreateUserRequest และ RegisterUserRequest มีโครงสร้างคล้ายกันมาก อาจจะใช้ตัวเดียวกันก็ได้ แต่แยกไว้ชัดเจนกว่า)
    let user_request = crate::domain::user::RegisterUserRequest {
        username: payload.username,
        password: payload.password,
    };

    match user_service.register_user(user_request).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(), // UserEntity (User) derive Serialize
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(), // <<-- เปลี่ยนเป็น BAD_REQUEST สำหรับ Error ทั่วไป เช่น username ซ้ำ
    }
}

// Handler สำหรับการ Login ผู้ใช้ (POST /login)
pub async fn login_user_handler(
    State(user_service): State<UserService>,
    Json(payload): Json<LoginUserRequest>, // รับ LoginUserRequest
) -> impl IntoResponse {
    // แปลง LoginUserRequest ไปเป็น LoginCredentials
    let login_credentials = LoginCredentials {
        username: payload.username,
        password: payload.password,
    };

    match user_service.login_user(login_credentials).await {
        Ok(user_id) => {
            let response = LoginResponse { user_id };
            (StatusCode::OK, Json(response)).into_response()
        },
        Err(e) => (StatusCode::UNAUTHORIZED, e).into_response(), // <<-- UNAUTHORIZED สำหรับ Login Failed
    }
}