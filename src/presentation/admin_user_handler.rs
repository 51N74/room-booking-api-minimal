// src/presentation/admin_user_handler.rs

use axum::{
    extract::{Path}, http::StatusCode, response::IntoResponse, Extension, Json
};
use std::sync::Arc;
use crate::{app_state::AppState};


// Handler สำหรับดึงข้อมูลผู้ใช้ทั้งหมด (Regular Users)
pub async fn get_all_users_handler(
    Extension(state): Extension<Arc<AppState>>, // <--- เปลี่ยนตรงนี้
) -> impl IntoResponse {
    match state.user_service.get_all_users().await {
        Ok(users) => Json(users).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}

pub async fn get_user_by_id_handler(
    Extension(state): Extension<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    match state.user_service.get_user_by_id(user_id).await {
        Ok(user) => Json(user).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}


// Handler สำหรับ Soft Delete ผู้ใช้โดย Admin
pub async fn delete_user_by_admin_handler(
    Extension(state): Extension<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    match state.user_service.delete_user(user_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            let status_code = if e.contains("User not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status_code, Json(serde_json::json!({"error": e}))).into_response()
        }
    }
}