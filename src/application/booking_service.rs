use crate::{
    domain::booking::{AddBookingEntity, BookingEntity},
    infrastructure::{
        booking_repository::BookingRepository
    },
};

use anyhow::Result;
pub struct BookingService {
    booking_repo: BookingRepository,
}

impl BookingService {
    pub fn new(booking_repo: BookingRepository) -> Self {
        BookingService { booking_repo }
    }

    pub async fn create_booking(&self, booking: AddBookingEntity) -> Result<BookingEntity> {
        self.booking_repo.create_booking(booking).await
    }

    pub async fn cancel_booking(&self, booking_id: i32) -> Result<BookingEntity> {
        self.booking_repo.cancel_booking(booking_id).await
    }
}
