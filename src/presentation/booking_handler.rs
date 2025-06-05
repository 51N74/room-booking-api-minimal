// src/presentation/booking_handler.rs

use std::sync::Arc;

use axum::{
    extract::{ Path, Extension},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde_json::json;
use crate::app_state::AppState;
use crate::application::booking_service::{BookingServiceError};
// import ให้ถูกต้องตามที่ใช้
use crate::domain::booking::{CreateBookingRequest, InternalCreateBookingRequest}; // เพิ่ม InternalCreateBookingRequest, CreateBookingRequest
use crate::infrastructure::jwt::Claims;


// Handler สำหรับสร้างการจองห้องพัก
// รับ CreateBookingRequest จาก Body
pub async fn create_booking_handler(
    Extension(state): Extension<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(create_request): Json<CreateBookingRequest>, // <--- เปลี่ยนเป็น CreateBookingRequest
) -> impl IntoResponse {
    let user_id_str = claims.sub;

    let user_id = match user_id_str.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Failed to parse user_id from claims: {}", user_id_str);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid user ID format in token."})),
            ).into_response();
        }
    };

    // สร้าง InternalCreateBookingRequest เพื่อส่งให้ Service
    let internal_request = InternalCreateBookingRequest {
        user_id, // ใช้ user_id จาก token
        room_id: create_request.room_id,
        start_time: create_request.start_time,
        end_time: create_request.end_time,
    };

    let booking_service = state.booking_service.clone(); // <--- เรียกจาก app_state โดยตรง

    match booking_service.create_booking(internal_request).await { // ส่ง internal_request
        Ok(booking) => (StatusCode::CREATED, Json(booking)).into_response(),
        Err(e) => match e {
            BookingServiceError::DbError(db_err) => {
                eprintln!("Database error creating booking: {}", db_err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Database error creating booking."})),
                ).into_response()
            },
            BookingServiceError::InvalidInput(msg) => {
                eprintln!("Invalid input creating booking: {}", msg);
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": format!("Invalid input: {}", msg)})),
                ).into_response()
            },
            BookingServiceError::NotFound => {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Room not found or unavailable."})),
                ).into_response()
            },
            BookingServiceError::Conflict => {
                (
                    StatusCode::CONFLICT,
                    Json(json!({"error": "Booking time conflict or room unavailable."})),
                ).into_response()
            }
            BookingServiceError::Unauthorized => { // ไม่ควรเกิดขึ้นตรงนี้ถ้า logic ถูกต้อง
                 (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Forbidden: Not authorized to create."})),
                ).into_response()
            }
        },
    }
}

// Handler สำหรับดึงการจองทั้งหมดของผู้ใช้ (โดยใช้ user_id จาก JWT)
pub async fn get_user_bookings_handler(
    Extension(state): Extension<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let user_id_str = claims.sub;

    let user_id = match user_id_str.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Failed to parse user_id from claims: {}", user_id_str);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid user ID format in token."})),
            ).into_response();
        }
    };

    let booking_service = state.booking_service.clone(); // <--- เรียกจาก app_state โดยตรง

    match booking_service.get_bookings_by_user_id(user_id).await {
        Ok(bookings) => (StatusCode::OK, Json(bookings)).into_response(),
        Err(e) => {
            eprintln!("Error getting user bookings: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to retrieve user bookings."})),
            ).into_response()
        }
    }
}

// Handler สำหรับยกเลิกการจอง
pub async fn cancel_booking_handler(
    Extension(state): Extension<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Path(booking_id): Path<i32>,
) -> impl IntoResponse {
    let user_id_str = claims.sub;
    let user_id = match user_id_str.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Failed to parse user_id from claims: {}", user_id_str);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid user ID format in token."})),
            ).into_response();
        }
    };

    let booking_service = state.booking_service.clone(); // <--- เรียกจาก app_state โดยตรง

    match booking_service.cancel_booking(booking_id, user_id).await {
        Ok(success) => {
            if success {
                StatusCode::NO_CONTENT.into_response()
            } else {
                // ถ้า affected_rows เป็น 0 อาจจะหมายถึง booking ไม่เจอหรือไม่ใช่ของ user นี้
                (
                    StatusCode::NOT_FOUND, // หรือ Forbidden ถ้าไม่ใช่เจ้าของ
                    Json(json!({"error": "Booking not found or not owned by user."})),
                ).into_response()
            }
        },
        Err(e) => match e {
            BookingServiceError::DbError(db_err) => {
                eprintln!("Database error canceling booking: {}", db_err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Database error canceling booking."})),
                ).into_response()
            },
            BookingServiceError::NotFound => {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Booking not found."})),
                ).into_response()
            },
            BookingServiceError::Unauthorized => { // ถ้า Service เช็คแล้วว่าไม่ได้รับอนุญาต
                 (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Forbidden: You do not own this booking."})),
                ).into_response()
            },
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to cancel booking."})),
            ).into_response()
        },
    }
}

// Handler สำหรับดึงการจองทั้งหมด (สำหรับ Admin)
pub async fn get_all_bookings_handler(
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let booking_service = state.booking_service.clone(); // <--- เรียกจาก app_state โดยตรง

    match booking_service.get_all_bookings().await {
        Ok(bookings) => (StatusCode::OK, Json(bookings)).into_response(),
        Err(e) => {
            eprintln!("Error getting all bookings: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to retrieve all bookings."})),
            ).into_response()
        }
    }
}

pub async fn delete_booking_handler(
    Extension(state): Extension<Arc<AppState>>, // Extracts shared application state.
    Path(booking_id): Path<i32>, // Extracts the booking ID from the URL path.
) -> impl IntoResponse {
    let booking_service = state.booking_service.clone(); // Clones the booking service.

    // Calls the booking service to perform the soft deletion of the booking.
    match booking_service.delete_booking(booking_id).await {
        Ok(booking) => {
            // On successful soft deletion, return 200 OK.
            // Since the operation performs a soft delete and returns the updated Booking object,
            // 200 OK is generally suitable. If no content were returned, 204 No Content would be preferred.
            (StatusCode::OK, Json(booking)).into_response()
        },
        Err(e) => {
            // Handle specific errors and return appropriate HTTP status codes.
            let error_message = format!("Failed to delete booking: {}", e);
            eprintln!("{}", error_message); // Log the error to stderr for debugging.

            // Check the type of error to return the correct HTTP status code.
            // It's recommended that `booking_service.delete_booking` returns a specific
            // custom error enum (e.g., `ServiceError::NotFound`, `ServiceError::DatabaseError`)
            // for more robust error handling instead of relying on string content.
            if e.to_string().contains("NotFound") { // This is a coarse check; prefer custom error types.
                (StatusCode::NOT_FOUND, error_message).into_response() // Return 404 if the booking isn't found.
            } else {
                // For other unhandled or unexpected errors (e.g., database issues),
                // return a 500 Internal Server Error.
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
        }
    }
}