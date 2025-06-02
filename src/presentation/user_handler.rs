// src/presentation/user_handler.rs

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};


use crate::application::user_service::UserService;
// นำเข้า Struct จาก Domain Layer
use crate::domain::user::LoginCredentials;
use crate::infrastructure::jwt::JwtService;

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
    pub token: String,      // เพิ่ม token ใน response
    pub role: String,       // เพิ่ม role ใน response
    pub expires_in: i64,    // เวลาหมดอายุ (วินาที)
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
    Json(payload): Json<LoginUserRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let login_credentials = LoginCredentials {
        username: payload.username,
        password: payload.password,
    };

    match user_service.login_user(login_credentials).await {
        Ok(user_id) => {
            // สร้าง JWT Token
            match JwtService::create_token(user_id, "user") {
                Ok(token) => {
                    let response = LoginResponse {
                        user_id,
                        token,
                        role: "user".to_string(),
                        expires_in: 24 * 60 * 60, // 24 ชั่วโมง
                    };
                    Ok(Json(response))
                }
                Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token".to_string())),
            }
        }
        Err(e) => Err((StatusCode::UNAUTHORIZED, e)),
    }
}