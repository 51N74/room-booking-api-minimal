// src/presentation/test_handler.rs

use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde_json::json;

// Handler ทดสอบสำหรับเส้นทางที่ต้องผ่าน auth_middleware (User)
pub async fn test_protected_user_route() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({"message": "Successfully accessed user-protected route!"})),
    )
}

// Handler ทดสอบสำหรับเส้นทางที่ต้องผ่าน admin_middleware (Admin)
pub async fn test_protected_admin_route() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({"message": "Successfully accessed admin-protected route!"})),
    )
}