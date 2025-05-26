use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::{application::user_service::UserService, domain::user::RegisterUserEntity};


#[derive(Clone,Deserialize)]
pub struct CreateUserRequest{
    pub username:String,
    pub password_hash:String,
}

pub async fn create_user_handler(
    State(state): State<UserService>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let user_dto = RegisterUserEntity {
        id:0,
        username: payload.username,
        password_hash: payload.password_hash,
    };

    match state.register_user(user_dto).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}