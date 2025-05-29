// src/application/booking_service.rs

use chrono::{NaiveDateTime, Utc};

use crate::domain::booking::{Booking, CreateBookingRequest, BookingStatus, NewBooking}; // <<-- ยังคง import BookingStatus และ NewBooking
use crate::infrastructure::booking_repository::BookingRepository;
use crate::infrastructure::room_repository::RoomRepository;

pub struct BookingService {
    booking_repo: BookingRepository,
    room_repo: RoomRepository,
}

impl BookingService {
    pub fn new(booking_repo: BookingRepository, room_repo: RoomRepository) -> Self {
        BookingService { booking_repo, room_repo }
    }

    pub async fn create_booking(&self, request: CreateBookingRequest) -> Result<Booking, String> {
        // ... (validation logic remains) ...
        let room = self.room_repo.get_room_by_id(request.room_id).await?;

        if request.start_time >= request.end_time {
            return Err("Start time must be before end time".to_string());
        }

        let overlapping_bookings = self.booking_repo.get_overlapping_bookings(
            request.room_id,
            request.start_time,
            request.end_time,
            None,
        ).await?;

        if !overlapping_bookings.is_empty() {
            return Err("Room is not available for the requested time slot".to_string());
        }

        let new_booking = NewBooking {
            room_id: request.room_id,
            user_id: request.user_id,
            start_time: request.start_time,
            end_time: request.end_time,
            status: BookingStatus::Pending, // <<-- ยังคงมี status
        };

        self.booking_repo.create_booking(new_booking).await
    }

    pub async fn get_booking_by_id(&self, booking_id: i32) -> Result<Booking, String> {
        self.booking_repo.get_booking_by_id(booking_id).await
    }

    pub async fn get_all_bookings(&self) -> Result<Vec<Booking>, String> {
        self.booking_repo.get_all_bookings().await
    }

    // Use Case: อัปเดตสถานะการจอง (ยังคงมี)
    pub async fn update_booking_status(&self, booking_id: i32, new_status: BookingStatus) -> Result<Booking, String> {
        let existing_booking = self.booking_repo.get_booking_by_id(booking_id).await?;

        // ... (business rules for status change remain) ...
        match (existing_booking.status, new_status.clone()) {
            (BookingStatus::Cancelled, _) => return Err("Cannot change status of a cancelled booking".to_string()),
            (_, BookingStatus::Pending) => return Err("Cannot revert to pending status from other states".to_string()),
            _ => { /* OK to proceed */ }
        }

        self.booking_repo.update_booking_status(booking_id, new_status).await
    }

    // Use Case: ยกเลิกการจอง (ยังคงมี)
    pub async fn cancel_booking(&self, booking_id: i32) -> Result<Booking, String> {
        self.update_booking_status(booking_id, BookingStatus::Cancelled).await
    }

    // Use Case: ยืนยันการจอง (ยังคงมี)
    pub async fn confirm_booking(&self, booking_id: i32) -> Result<Booking, String> {
        self.update_booking_status(booking_id, BookingStatus::Confirmed).await
    }
}