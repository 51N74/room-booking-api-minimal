use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::{application::{admin_service::AdminService}, domain::{admin::RegisterAdminEntity}};


#[derive(Clone,Deserialize)]
pub struct CreateAdminRequest{
    pub username:String,
    pub password_hash:String,
}

pub async fn create_admin_handler(
    State(state): State<AdminService>,
    Json(payload): Json<CreateAdminRequest>,
) -> impl IntoResponse {
    let admin_dto = RegisterAdminEntity {
        id:0,
        username: payload.username,
        password_hash: payload.password_hash,
    };

    match state.register_admin(admin_dto).await {
        Ok(admin) => (StatusCode::CREATED, Json(admin)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}