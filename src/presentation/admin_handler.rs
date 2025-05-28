// src/presentation/user_handler.rs

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::application::admin_service::AdminService;

// นำเข้า Struct จาก Domain Layer
use crate::domain::admin::LoginCredentials;

// Request Body สำหรับการลงทะเบียน (รับรหัสผ่านดิบจาก Client)
#[derive(Clone, Deserialize)]
pub struct CreateAdminRequest {
    pub username: String,
    pub password: String, // <<-- รหัสผ่านดิบ (Plain Text)
}

// Request Body สำหรับการ Login (รับรหัสผ่านดิบจาก Client)
#[derive(Clone, Deserialize)]
pub struct LoginAdminRequest {
    pub username: String,
    pub password: String, // <<-- รหัสผ่านดิบ (Plain Text)
}

// Response Body สำหรับการ Login (ถ้าต้องการส่ง Token หรือข้อมูลเพิ่มเติม)
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub admin_id: i32,
    // pub token: String, // ถ้ามี Token-based authentication
}

// Handler สำหรับการลงทะเบียนผู้ใช้ (POST /register)
pub async fn register_admin_handler(
    State(admin_service): State<AdminService>,
    Json(payload): Json<CreateAdminRequest>, // รับ CreateUserRequest
) -> impl IntoResponse {
    // แปลง CreateUserRequest ไปเป็น RegisterUserRequest
    // (ตอนนี้ CreateUserRequest และ RegisterUserRequest มีโครงสร้างคล้ายกันมาก อาจจะใช้ตัวเดียวกันก็ได้ แต่แยกไว้ชัดเจนกว่า)
    let admin_request = crate::domain::admin::RegisterAdminRequest {
        username: payload.username,
        password: payload.password,
    };

    match admin_service.register_admin(admin_request).await {
        Ok(admin) => (StatusCode::CREATED, Json(admin)).into_response(), // UserEntity (User) derive Serialize
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(), // <<-- เปลี่ยนเป็น BAD_REQUEST สำหรับ Error ทั่วไป เช่น username ซ้ำ
    }
}

// Handler สำหรับการ Login ผู้ใช้ (POST /login)
pub async fn login_admin_handler(
    State(admin_service): State<AdminService>,
    Json(payload): Json<LoginAdminRequest>, // รับ LoginUserRequest
) -> impl IntoResponse {
    // แปลง LoginUserRequest ไปเป็น LoginCredentials
    let login_credentials = LoginCredentials {
        username: payload.username,
        password: payload.password,
    };

    match admin_service.login_admin(login_credentials).await {
        Ok(admin_id) => {
            let response = LoginResponse { admin_id };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e).into_response(), // <<-- UNAUTHORIZED สำหรับ Login Failed
    }
}
