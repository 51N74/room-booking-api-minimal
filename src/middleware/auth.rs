// src/middleware/auth.rs
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use serde_json::json;
use crate::infrastructure::jwt::{JwtService, Claims};

// Middleware สำหรับตรวจสอบว่า User ล็อกอินแล้วหรือไม่
pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // ดึง Authorization header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            &header[7..] // ตัด "Bearer " ออก
        }
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Missing or invalid authorization header"})),
            ));
        }
    };

    // ตรวจสอบ Token
    match JwtService::verify_token(token) {
        Ok(token_data) => {
            // เพิ่ม Claims ลงใน Request Extensions
            req.extensions_mut().insert(token_data.claims);
            Ok(next.run(req).await)
        }
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid or expired token"})),
        )),
    }
}

// Middleware สำหรับตรวจสอบว่าเป็น Admin หรือไม่
pub async fn admin_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // ดึง Authorization header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            &header[7..] // ตัด "Bearer " ออก
        }
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Missing or invalid authorization header"})),
            ));
        }
    };

    // ตรวจสอบ Token
    match JwtService::verify_token(token) {
        Ok(token_data) => {
            // ตรวจสอบว่าเป็น Admin หรือไม่
            if token_data.claims.role != "admin" {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Admin access required"})),
                ));
            }
            
            // เพิ่ม Claims ลงใน Request Extensions
            req.extensions_mut().insert(token_data.claims);
            Ok(next.run(req).await)
        }
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid or expired token"})),
        )),
    }
}
