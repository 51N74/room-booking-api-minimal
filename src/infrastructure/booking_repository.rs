use crate::domain::booking::{Booking, InternalCreateBookingRequest, NewBooking};
use crate::infrastructure::room_repository::RoomRepository;
use crate::infrastructure::schema::bookings;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::Utc;
#[derive(Clone)]
pub struct BookingRepository;

    impl BookingRepository {
    pub fn create_booking(
        conn: &mut SqliteConnection, // A mutable reference to the SQLite database connection.
        request: InternalCreateBookingRequest,
    ) -> Result<Booking, diesel::result::Error> {
        // --- Begin Database Transaction ---
        // All operations within this block are treated as a single atomic unit.
        // If any step fails, the entire transaction will be rolled back.
        conn.transaction(|transaction_conn| {
            // Create a new booking record with the provided details and current timestamps.
            let new_booking = NewBooking {
                room_id: request.room_id,
                user_id: request.user_id,
                start_time: request.start_time.naive_utc(),
                end_time: request.end_time.naive_utc(),
                status: "active".to_string(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                deleted_at: None,
            };

            // Insert the new booking into the database.
            diesel::insert_into(bookings::table)
                .values(&new_booking)
                .execute(transaction_conn)?; // Use `transaction_conn` for the atomic operation.

            // Retrieve the newly created booking by selecting the most recent record.
            let created_booking = bookings::table
                .order(bookings::id.desc())
                .select(Booking::as_select())
                .first(transaction_conn)?; // Use `transaction_conn` for the atomic operation.

            // --- Update Room Status within the same transaction ---
            // Call the synchronous room status update function, ensuring it uses the
            // same transaction connection for atomicity.
            RoomRepository::update_room_status_sync(
                transaction_conn, // Pass the transaction's connection.
                request.room_id,
                "booked", // Set the room status to "booked".
            )?;
            // --- End Room Status Update ---

            Ok(created_booking) // Return the successfully created booking.
        }) // The transaction will commit here if all operations succeed, or rollback on error.
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
