use crate::domain::booking::{Booking, CreateBookingRequest, InternalCreateBookingRequest, NewBooking};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::Utc;

pub struct BookingRepository;

impl BookingRepository {
     // สร้างการจองใหม่ - ใช้ InternalCreateBookingRequest
    pub fn create_booking(
        conn: &mut SqliteConnection,
        request: InternalCreateBookingRequest,
    ) -> Result<Booking, diesel::result::Error> {
        use crate::infrastructure::schema::bookings;
        
        let new_booking = NewBooking {
            room_id: request.room_id,
            user_id: request.user_id,
            start_time: request.start_time.naive_utc(),
            end_time: request.end_time.naive_utc(),
            status: "active".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        diesel::insert_into(bookings::table)
            .values(&new_booking)
            .execute(conn)?;

        // ดึงข้อมูลการจองที่เพิ่งสร้าง
        bookings::table
            .order(bookings::id.desc())
            .select(Booking::as_select())
            .first(conn)
    }

    // ยกเลิกการจอง (Soft Delete)
    pub fn cancel_booking(
        conn: &mut SqliteConnection,
        booking_id: i32,
        user_id: i32,
    ) -> Result<bool, diesel::result::Error> {
        use crate::infrastructure::schema::bookings;

        let affected_rows = diesel::update(
            bookings::table
                .filter(bookings::id.eq(booking_id))
                .filter(bookings::user_id.eq(user_id))
                .filter(bookings::deleted_at.is_null())
        )
        .set((
            bookings::status.eq("cancelled"),
            bookings::deleted_at.eq(Some(Utc::now().naive_utc())),
            bookings::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)?;

        Ok(affected_rows > 0)
    }

    // ดึงการจองทั้งหมดของผู้ใช้
    pub fn get_user_bookings(
        conn: &mut SqliteConnection,
        user_id: i32,
    ) -> Result<Vec<Booking>, diesel::result::Error> {
        use crate::infrastructure::schema::bookings;

        bookings::table
            .filter(bookings::user_id.eq(user_id))
            .filter(bookings::deleted_at.is_null())
            .order(bookings::created_at.desc())
            .select(Booking::as_select())
            .load(conn)
    }

    // ดึงการจองทั้งหมด (สำหรับ Admin)
    pub fn get_all_bookings(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Booking>, diesel::result::Error> {
        use crate::infrastructure::schema::bookings;

        bookings::table
            .filter(bookings::deleted_at.is_null())
            .order(bookings::created_at.desc())
            .select(Booking::as_select())
            .load(conn)
    }

    // ดึงการจองตาม ID
    pub fn get_booking_by_id(
        conn: &mut SqliteConnection,
        booking_id: i32,
    ) -> Result<Option<Booking>, diesel::result::Error> {
        use crate::infrastructure::schema::bookings;

        bookings::table
            .filter(bookings::id.eq(booking_id))
            .filter(bookings::deleted_at.is_null())
            .select(Booking::as_select())
            .first(conn)
            .optional()
    }
}
