// src/infrastructure/booking_repository.rs

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use chrono::{NaiveDateTime, Utc};

use crate::infrastructure::schema::bookings;
use crate::domain::booking::{Booking, NewBooking, BookingChangeset, BookingStatus}; // <<-- ยังคง import BookingStatus

pub struct BookingRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl BookingRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        BookingRepository { pool }
    }

    pub async fn create_booking(&self, new_booking_data: NewBooking) -> Result<Booking, String> {
        let mut conn = self.pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

        diesel::insert_into(bookings::table)
            .values(&new_booking_data)
            .execute(&mut conn)
            .map_err(|e| format!("Failed to insert booking into DB: {}", e))?;

        bookings::table
            .filter(bookings::room_id.eq(new_booking_data.room_id))
            .filter(bookings::user_id.eq(new_booking_data.user_id))
            .filter(bookings::start_time.eq(new_booking_data.start_time))
            .filter(bookings::end_time.eq(new_booking_data.end_time))
            .order(bookings::created_at.desc())
            .first::<Booking>(&mut conn)
            .map_err(|e| format!("Failed to retrieve newly created booking: {}", e))
    }

    pub async fn get_booking_by_id(&self, booking_id: i32) -> Result<Booking, String> {
        let mut conn = self.pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

        bookings::table
            .filter(bookings::id.eq(booking_id))
            .first::<Booking>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => "Booking not found".to_string(),
                _ => format!("Failed to retrieve booking by ID: {}", e),
            })
    }

    pub async fn get_all_bookings(&self) -> Result<Vec<Booking>, String> {
        let mut conn = self.pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

        bookings::table
            .load::<Booking>(&mut conn)
            .map_err(|e| format!("Failed to retrieve all bookings: {}", e))
    }

    pub async fn get_overlapping_bookings(
        &self,
        room_id: i32,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        exclude_booking_id: Option<i32>,
    ) -> Result<Vec<Booking>, String> {
        let mut conn = self.pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

        let mut query = bookings::table
            .filter(bookings::room_id.eq(room_id))
            .filter(bookings::status.ne(BookingStatus::Cancelled)) // <<-- ยังคงมี filter สถานะ
            .filter(bookings::status.ne(BookingStatus::Completed)) // <<-- ยังคงมี filter สถานะ
            .filter(bookings::start_time.lt(end_time))
            .filter(bookings::end_time.gt(start_time))
            .into_boxed();

        if let Some(id) = exclude_booking_id {
            query = query.filter(bookings::id.ne(id));
        }

        query.load::<Booking>(&mut conn)
            .map_err(|e| format!("Failed to check for overlapping bookings: {}", e))
    }

    // ฟังก์ชันอัปเดตสถานะการจอง (ยังคงมี)
    pub async fn update_booking_status(&self, booking_id: i32, new_status: BookingStatus) -> Result<Booking, String> {
        let mut conn = self.pool.get().map_err(|e| format!("Failed to get DB connection: {}", e))?;

        let changes = BookingChangeset {
            room_id: None,
            user_id: None,
            start_time: None,
            end_time: None,
            status: Some(new_status), // <<-- ยังคงมี status
            updated_at: Some(Utc::now().naive_utc()),
        };

        let updated_rows = diesel::update(bookings::table.filter(bookings::id.eq(booking_id)))
            .set(changes)
            .execute(&mut conn)
            .map_err(|e| format!("Failed to update booking status: {}", e))?;

        if updated_rows == 0 {
            return Err("Booking not found or no changes applied".to_string());
        }

        bookings::table
            .filter(bookings::id.eq(booking_id))
            .first::<Booking>(&mut conn)
            .map_err(|e| format!("Failed to retrieve updated booking: {}", e))
    }
}