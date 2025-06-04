// src/application/booking_service.rs

use crate::domain::booking::{Booking,InternalCreateBookingRequest}; // เพิ่ม InternalCreateBookingRequest
use crate::infrastructure::booking_repository::BookingRepository;
use crate::infrastructure::database::DbPool; // ต้อง import DbPool
use diesel::sqlite::SqliteConnection; // ต้อง import SqliteConnection
use diesel::r2d2::PooledConnection; // ต้อง import PooledConnection

#[derive(Debug)]
pub enum BookingServiceError {
    DbError(String),
    InvalidInput(String),
    NotFound,
    Conflict,
    Unauthorized,
}

impl std::fmt::Display for BookingServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookingServiceError::DbError(msg) => write!(f, "Database error: {}", msg),
            BookingServiceError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            BookingServiceError::NotFound => write!(f, "Not found"),
            BookingServiceError::Conflict => write!(f, "Conflict"),
            BookingServiceError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl From<diesel::result::Error> for BookingServiceError {
    fn from(error: diesel::result::Error) -> Self {
        BookingServiceError::DbError(error.to_string())
    }
}

impl From<r2d2::Error> for BookingServiceError {
    fn from(error: r2d2::Error) -> Self {
        BookingServiceError::DbError(format!("Connection pool error: {}", error))
    }
}


#[derive(Clone)]
pub struct BookingService {
    pool: DbPool, // BookingService จะเก็บ DbPool แทน BookingRepository
}

impl BookingService {
    // *** แก้ไข new() ให้รับ DbPool ***
    pub fn new(pool: DbPool) -> Self {
        BookingService { pool }
    }

    fn get_connection(&self) -> Result<PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>, BookingServiceError> {
        self.pool.get().map_err(|e| BookingServiceError::DbError(format!("Failed to get DB connection: {}", e)))
    }

    // *** แก้ไข create_booking ให้รับ InternalCreateBookingRequest ***
    // NewBooking ควรสร้างใน Repository หรือ Service ก่อนส่งให้ Repository
    pub async fn create_booking(&self, request: InternalCreateBookingRequest) -> Result<Booking, BookingServiceError> {
        let conn = &mut self.get_connection()?; // ดึง Connection
        // เนื่องจาก BookingRepository ไม่มี state เราสามารถเรียกใช้ method ได้เลย
        BookingRepository::create_booking(conn, request)
            .map_err(|e| BookingServiceError::DbError(e.to_string()))
    }

    pub async fn get_bookings_by_user_id(&self, user_id: i32) -> Result<Vec<Booking>, BookingServiceError> {
        let conn = &mut self.get_connection()?;
        BookingRepository::get_user_bookings(conn, user_id)
            .map_err(|e| BookingServiceError::DbError(e.to_string()))
    }

    pub async fn cancel_booking(&self, booking_id: i32, user_id: i32) -> Result<bool, BookingServiceError> {
        let conn = &mut self.get_connection()?;
        // ใน BookingRepository คุณมีการ filter user_id ใน cancel_booking อยู่แล้ว
        // ดังนั้นตรงนี้แค่เรียกใช้ได้เลย
        BookingRepository::cancel_booking(conn, booking_id, user_id)
            .map_err(|e| BookingServiceError::DbError(e.to_string())) // ควรจัดการ NotFound/Unauthorized ใน Service
    }

    pub async fn get_all_bookings(&self) -> Result<Vec<Booking>, BookingServiceError> {
        let conn = &mut self.get_connection()?;
        BookingRepository::get_all_bookings(conn)
            .map_err(|e| BookingServiceError::DbError(e.to_string()))
    }
}