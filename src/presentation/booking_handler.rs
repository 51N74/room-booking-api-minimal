use axum::{
    extract::{Path, State, Extension},
    response::{IntoResponse, Json}, // เพิ่ม IntoResponse
    http::StatusCode,
};
use serde_json::{json, Value};
use crate::{app_state::AppState, application::booking_service::BookingService, domain::booking::InternalCreateBookingRequest};
use crate::domain::booking::{CreateBookingRequest, CancelBookingRequest, Booking};
use crate::infrastructure::database::DbPool;
use crate::infrastructure::jwt::Claims;

// POST /bookings - สร้างการจอง (ต้อง Login)
// เพิ่มสำหรับ Debug
pub async fn create_booking_handler(
    State(app_state): State<AppState>, // *** เป็น AppState ***
    Extension(claims): Extension<Claims>, // ดึง Claims จาก middleware
    Json(mut request): Json<InternalCreateBookingRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // ใช้ user_id จาก Token แทนที่จะรับจาก request
    let user_id: i32 = claims.sub.parse().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid user ID in token"})))
    })?;
    
    request.user_id = user_id; // กำหนด user_id จาก token
    
    let mut conn = app_state.db_pool.get().map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "ไม่สามารถเชื่อมต่อฐานข้อมูลได้"})))
    })?;
    
    match BookingService::create_booking(&mut conn, request) {
        Ok(booking) => Ok(Json(json!(booking))),
        Err(err) => Err((StatusCode::BAD_REQUEST, Json(json!({"error": err})))),
    }
}

// DELETE /bookings/{id} - ยกเลิกการจอง (ต้อง Login)
pub async fn cancel_booking_handler(
    State(app_state): State<AppState>, // *** เป็น AppState ***
    Extension(claims): Extension<Claims>,
    Path(booking_id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let user_id: i32 = claims.sub.parse().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid user ID in token"})))
    })?;
    
    let cancel_request = CancelBookingRequest { user_id };
    
    let mut conn = app_state.db_pool.get().map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "ไม่สามารถเชื่อมต่อฐานข้อมูลได้"})))
    })?;
    
    match BookingService::cancel_booking(&mut conn, booking_id, cancel_request) {
        Ok(true) => Ok(Json(json!({"message": "ยกเลิกการจองสำเร็จ"}))),
        Ok(false) => Err((StatusCode::NOT_FOUND, Json(json!({"error": "ไม่พบการจองหรือไม่มีสิทธิ์ยกเลิก"})))),
        Err(err) => Err((StatusCode::BAD_REQUEST, Json(json!({"error": err})))),
    }
}

// GET /bookings/user/{user_id} - ดึงการจองของผู้ใช้ (ต้อง Login)
pub async fn get_user_bookings_handler(
     State(app_state): State<AppState>, // *** เป็น AppState ***
    Extension(claims): Extension<Claims>,
    Path(requested_user_id): Path<i32>,
) -> Result<Json<Vec<Booking>>, (StatusCode, Json<Value>)> {
    let user_id: i32 = claims.sub.parse().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid user ID in token"})))
    })?;
    
    // ผู้ใช้สามารถดูการจองของตัวเองเท่านั้น หรือเป็น Admin
    if user_id != requested_user_id && claims.role != "admin" {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Access denied"}))));
    }
    
    let mut conn = app_state.db_pool.get().map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "ไม่สามารถเชื่อมต่อฐานข้อมูลได้"})))
    })?;
    
    match BookingService::get_user_bookings(&mut conn, requested_user_id) {
        Ok(bookings) => Ok(Json(bookings)),
        Err(err) => Err((StatusCode::BAD_REQUEST, Json(json!({"error": err})))),
    }
}

// GET /bookings - ดึงการจองทั้งหมด (Admin เท่านั้น)
pub async fn get_all_bookings_handler(
    // *** เปลี่ยนจาก State(pool): State<DbPool> ***
    State(app_state): State<AppState>, // *** เป็น State(app_state): State<AppState> ***
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Booking>>, (StatusCode, Json<Value>)> {
    if claims.role != "admin" {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Access denied: Admin role required."}))));
    }

    // *** ดึง db_pool จาก app_state ***
    let mut conn = app_state.db_pool.get().map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "ไม่สามารถเชื่อมต่อฐานข้อมูลได้"})))
    })?;

    match BookingService::get_all_bookings(&mut conn) {
        Ok(bookings) => Ok(Json(bookings)),
        Err(err) => Err((StatusCode::BAD_REQUEST, Json(json!({"error": err.to_string()})))),
    }
}