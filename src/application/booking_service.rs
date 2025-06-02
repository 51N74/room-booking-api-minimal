// src/application/booking_service.rs
use crate::domain::booking::{Booking, CancelBookingRequest, CreateBookingRequest, InternalCreateBookingRequest};
use crate::infrastructure::booking_repository::BookingRepository;
use diesel::sqlite::SqliteConnection;

pub struct BookingService;

impl BookingService {
     // สร้างการจอง - ใช้ InternalCreateBookingRequest แทน
    pub fn create_booking(
        conn: &mut SqliteConnection,
        request: InternalCreateBookingRequest,
    ) -> Result<Booking, String> {
        // ตรวจสอบพื้นฐาน
        if request.start_time >= request.end_time {
            return Err("เวลาเริ่มต้องมาก่อนเวลาสิ้นสุด".to_string());
        }

        BookingRepository::create_booking(conn, request)
            .map_err(|e| format!("ไม่สามารถสร้างการจองได้: {}", e))
    }

    // ยกเลิกการจอง
    pub fn cancel_booking(
        conn: &mut SqliteConnection,
        booking_id: i32,
        request: CancelBookingRequest,
    ) -> Result<bool, String> {
        BookingRepository::cancel_booking(conn, booking_id, request.user_id)
            .map_err(|e| format!("ไม่สามารถยกเลิกการจองได้: {}", e))
    }

    // ดึงการจองของผู้ใช้
    pub fn get_user_bookings(
        conn: &mut SqliteConnection,
        user_id: i32,
    ) -> Result<Vec<Booking>, String> {
        BookingRepository::get_user_bookings(conn, user_id)
            .map_err(|e| format!("ไม่สามารถดึงข้อมูลการจองได้: {}", e))
    }

    // ดึงการจองทั้งหมด
    pub fn get_all_bookings(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Booking>, String> {
        BookingRepository::get_all_bookings(conn)
            .map_err(|e| format!("ไม่สามารถดึงข้อมูลการจองทั้งหมดได้: {}", e))
    }
}