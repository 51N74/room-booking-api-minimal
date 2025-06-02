// src/domain/booking.rs
use crate::infrastructure::schema::bookings;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = bookings)]
pub struct Booking {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = bookings)]
pub struct NewBooking {
    pub room_id: i32,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    pub room_id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CancelBookingRequest {
    pub user_id: i32,
}
#[derive(Debug, Deserialize)]
pub struct InternalCreateBookingRequest {
    pub room_id: i32,
    pub user_id: i32, // จะได้จาก JWT Token
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}
