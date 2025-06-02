use axum::{
    extract::{Path, State},
    response::Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use crate::application::booking_service::BookingService;
use crate::domain::booking::{CreateBookingRequest, CancelBookingRequest, Booking};
use crate::infrastructure::database::DbPool;

// POST /bookings - สร้างการจอง
pub async fn create_booking_handler(
    State(pool): State<DbPool>,
    Json(request): Json<CreateBookingRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let mut conn = pool.get().map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "ไม่สามารถเชื่อมต่อฐานข้อมูลได้"})))
    })?;
    
    match BookingService::create_booking(&mut conn, request) {
        Ok(booking) => Ok(Json(json!(booking))),
        Err(err) => Err((StatusCode::BAD_REQUEST, Json(json!({"error": err})))),
    }
}

// DELETE /bookings/{id} - ยกเลิกการจอง
pub async fn cancel_booking_handler(
    State(pool): State<DbPool>,
    Path(booking_id): Path<i32>,
    Json(request): Json<CancelBookingRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let mut conn = pool.get().map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "ไม่สามารถเชื่อมต่อฐานข้อมูลได้"})))
    })?;
    
    match BookingService::cancel_booking(&mut conn, booking_id, request) {
        Ok(true) => Ok(Json(json!({"message": "ยกเลิกการจองสำเร็จ"}))),
        Ok(false) => Err((StatusCode::NOT_FOUND, Json(json!({"error": "ไม่พบการจองหรือไม่มีสิทธิ์ยกเลิก"})))),
        Err(err) => Err((StatusCode::BAD_REQUEST, Json(json!({"error": err})))),
    }
}

// GET /bookings/user/{user_id} - ดึงการจองของผู้ใช้
pub async fn get_user_bookings_handler(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Booking>>, (StatusCode, Json<Value>)> {
    let mut conn = pool.get().map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "ไม่สามารถเชื่อมต่อฐานข้อมูลได้"})))
    })?;
    
    match BookingService::get_user_bookings(&mut conn, user_id) {
        Ok(bookings) => Ok(Json(bookings)),
        Err(err) => Err((StatusCode::BAD_REQUEST, Json(json!({"error": err})))),
    }
}

// GET /bookings - ดึงการจองทั้งหมด (สำหรับ Admin)
pub async fn get_all_bookings_handler(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Booking>>, (StatusCode, Json<Value>)> {
    let mut conn = pool.get().map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "ไม่สามารถเชื่อมต่อฐานข้อมูลได้"})))
    })?;
    
    match BookingService::get_all_bookings(&mut conn) {
        Ok(bookings) => Ok(Json(bookings)),
        Err(err) => Err((StatusCode::BAD_REQUEST, Json(json!({"error": err})))),
    }
}