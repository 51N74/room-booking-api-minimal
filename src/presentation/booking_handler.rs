// src/presentation/booking_handler.rs

use axum::{extract::{State, Path}, Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::application::booking_service::BookingService;
use crate::domain::booking::{Booking, CreateBookingRequest, BookingStatus}; // <<-- ยังคง import BookingStatus

// Request Body สำหรับการสร้างการจอง (ยังคงมี)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateBookingPayload {
    pub room_id: i32,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

// Request Body สำหรับการอัปเดตสถานะการจอง (ยังคงมี)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateBookingStatusPayload {
    pub status: BookingStatus, // <<-- ยังคงมี status
}

// Handler สำหรับสร้างการจองใหม่ (POST /bookings) (ยังคงมี)
pub async fn create_booking_handler(
    State(booking_service): State<BookingService>,
    Json(payload): Json<CreateBookingPayload>,
) -> impl IntoResponse {
    let request = CreateBookingRequest {
        room_id: payload.room_id,
        user_id: payload.user_id,
        start_time: payload.start_time,
        end_time: payload.end_time,
    };

    match booking_service.create_booking(request).await {
        Ok(booking) => (StatusCode::CREATED, Json(booking)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

// Handler สำหรับดึงการจองตาม ID (GET /bookings/:id) (ยังคงมี)
pub async fn get_booking_by_id_handler(
    State(booking_service): State<BookingService>,
    Path(booking_id): Path<i32>,
) -> impl IntoResponse {
    match booking_service.get_booking_by_id(booking_id).await {
        Ok(booking) => (StatusCode::OK, Json(booking)).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e).into_response(),
    }
}

// Handler สำหรับดึงการจองทั้งหมด (GET /bookings) (ยังคงมี)
pub async fn get_all_bookings_handler(
    State(booking_service): State<BookingService>,
) -> impl IntoResponse {
    match booking_service.get_all_bookings().await {
        Ok(bookings) => (StatusCode::OK, Json(bookings)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler สำหรับอัปเดตสถานะการจอง (PUT /bookings/:id/status) (ยังคงมี)
pub async fn update_booking_status_handler(
    State(booking_service): State<BookingService>,
    Path(booking_id): Path<i32>,
    Json(payload): Json<UpdateBookingStatusPayload>,
) -> impl IntoResponse {
    match booking_service.update_booking_status(booking_id, payload.status).await {
        Ok(booking) => (StatusCode::OK, Json(booking)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

// Handler สำหรับยกเลิกการจอง (PUT /bookings/:id/cancel) (ยังคงมี)
pub async fn cancel_booking_handler(
    State(booking_service): State<BookingService>,
    Path(booking_id): Path<i32>,
) -> impl IntoResponse {
    match booking_service.cancel_booking(booking_id).await {
        Ok(booking) => (StatusCode::OK, Json(booking)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

// Handler สำหรับยืนยันการจอง (PUT /bookings/:id/confirm) (ยังคงมี)
pub async fn confirm_booking_handler(
    State(booking_service): State<BookingService>,
    Path(booking_id): Path<i32>,
) -> impl IntoResponse {
    match booking_service.confirm_booking(booking_id).await {
        Ok(booking) => (StatusCode::OK, Json(booking)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}