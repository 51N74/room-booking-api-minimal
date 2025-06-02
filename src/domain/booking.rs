// src/domain/booking.rs
use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::infrastructure::schema::bookings;

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
    pub user_id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CancelBookingRequest {
    pub user_id: i32,
}