// src/presentation/admin_handler.rs

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json; // เพิ่ม import สำหรับ json! macro

use crate::app_state::AppState;
// use crate::application::admin_service::AdminService; // ไม่จำเป็นต้องใช้ตรงๆ แล้ว เพราะเข้าถึงผ่าน AppState
use crate::domain::admin::{LoginCredentials}; // นำเข้า Struct จาก Domain Layer


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
    pub token: String,      // เพิ่ม token ใน response
    pub role: String,       // เพิ่ม role ใน response
    pub expires_in: i64,    // เวลาหมดอายุ (วินาที)
}

// Handler สำหรับการลงทะเบียนผู้ใช้ (POST /register)
pub async fn register_admin_handler(
    // *** ลบ State ซ้ำซ้อนออกไป เหลือแค่ 1 อัน ***
    State(app_state): State<AppState>,
    Json(payload): Json<CreateAdminRequest>,
) -> impl IntoResponse {
    // 1. ดึง Connection จาก AppState
    let mut conn = match app_state.db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to connect to database."}))).into_response();
        }
    };

    // 2. แปลง CreateAdminRequest ไปเป็น RegisterAdminRequest
    let admin_request = crate::domain::admin::RegisterAdminRequest {
        username: payload.username,
        password: payload.password,
    };

    // 3. เรียกใช้ service และจัดการผลลัพธ์
    // *** ต้องส่ง &mut conn เข้าไปในพารามิเตอร์แรกของ register_admin ***
    match app_state.admin_service.register_admin(&mut conn, admin_request).await {
        Ok(admin) => (StatusCode::CREATED, Json(json!(admin))).into_response(), // UserEntity (User) derive Serialize
        Err(e) => {
            // *** แก้ไขการจัดการ Error ***
            eprintln!("Admin registration error: {:?}", e); // สำหรับ Debug
            // สมมติว่า AdminServiceError มี to_string() หรือ Display trait
            (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))).into_response()
            // ถ้าคุณมี AdminServiceError enum ที่ละเอียดกว่านี้ เช่น UsernameAlreadyExists
            // คุณสามารถใช้ match e { ... } เพื่อจัดการแต่ละ error ได้แม่นยำขึ้น
        }
    }
}

// Handler สำหรับการ Login Admin (POST /login/admin)
pub async fn login_admin_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginAdminRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let login_credentials = LoginCredentials {
        username: payload.username,
        password: payload.password,
    };

    // ดึง connection สำหรับ login_admin
    let mut conn = match app_state.db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to connect to database.".to_string()));
        }
    };

    // *** ต้องส่ง &mut conn เข้าไปในพารามิเตอร์แรกของ login_admin ***
    match app_state.admin_service.login_admin(&mut conn, login_credentials).await {
        Ok(admin_id) => {
            // สร้าง JWT Token สำหรับ Admin
            match app_state.jwt_service.create_token(admin_id, "admin") {
                Ok(token) => {
                    let response = LoginResponse {
                        admin_id,
                        token,
                        role: "admin".to_string(),
                        expires_in: 24 * 60 * 60, // 24 ชั่วโมง
                    };
                    Ok(Json(response))
                }
                Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token".to_string())),
            }
        }
        Err(e) => Err((StatusCode::UNAUTHORIZED, e.to_string())), // แปลง error เป็น String
    }
}