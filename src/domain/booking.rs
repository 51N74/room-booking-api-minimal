// src/domain/booking.rs

use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use std::fmt;

use crate::infrastructure::schema::bookings;

// Enum สำหรับสถานะการจอง (ยังคงมี)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::infrastructure::schema::sql_types::BookingStatus"]
pub enum BookingStatus {
    #[db_rename = "pending"]
    Pending,
    #[db_rename = "confirmed"]
    Confirmed,
    #[db_rename = "cancelled"]
    Cancelled,
    #[db_rename = "completed"]
    Completed,
}

impl fmt::Display for BookingStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Booking Entity (ยังคงมี status)
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
#[diesel(table_name = bookings)]
pub struct Booking {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub status: BookingStatus, // <<-- ยังคงมี status
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// CreateBookingRequest (ยังคงมี status)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBookingRequest {
    pub room_id: i32,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

// NewBooking (ยังคงมี status)
#[derive(Debug, Insertable)]
#[diesel(table_name = bookings)]
pub struct NewBooking {
    pub room_id: i32,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub status: BookingStatus, // <<-- ยังคงมี status
}

// BookingChangeset (ยังคงมี status)
#[derive(Debug, Clone, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = bookings)]
pub struct BookingChangeset {
    pub room_id: Option<i32>,
    pub user_id: Option<i32>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub status: Option<BookingStatus>, // <<-- ยังคงมี status
    pub updated_at: Option<NaiveDateTime>,
}

// UpdateBookingRequest (ยังคงมี status)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateBookingRequest {
    pub room_id: Option<i32>,
    pub user_id: Option<i32>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub status: Option<BookingStatus>, // <<-- ยังคงมี status
}