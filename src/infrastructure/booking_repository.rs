// use crate::domain::booking::AddBookingEntity;
// use crate::domain::booking::BookingEntity;
// use anyhow::Ok;
// use anyhow::Result;
// use diesel::SqliteConnection;
// use diesel::prelude::*;

// use super::schema::bookings;
// #[derive(Debug, Clone)]
// pub struct BookingRepository {
//     pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>,
// }

// impl BookingRepository {
//     pub fn new(database_url: &str) -> Self {
//         let manager = diesel::r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
//         let pool = diesel::r2d2::Pool::builder()
//             .build(manager)
//             .expect("Failed to create pool");
//         BookingRepository { pool }
//     }

//     pub async fn create_booking(&self, booking: AddBookingEntity) -> Result<BookingEntity> {
//         let new_booking = (
//             bookings::user_id.eq(booking.user_id),
//             bookings::room_id.eq(booking.room_id),
//             bookings::start_time.eq(booking.start_time),
//             bookings::end_time.eq(booking.end_time),
//         );

//         let mut conn = self.pool.get()?;

//         diesel::insert_into(bookings::table)
//             .values(&new_booking)
//             .execute(&mut conn)
//             .map_err(|e| anyhow::anyhow!("Failed to insert booking: {}", e))?;

//         let id = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
//             "last_insert_rowid()",
//         ))
//         .get_result::<i32>(&mut conn)
//         .map_err(|e| anyhow::anyhow!("Failed to get last insert id: {}", e))?;

//         Ok(BookingEntity {
//             id,
//             user_id: booking.user_id,
//             room_id: booking.room_id,
//             start_time: booking.start_time,
//             end_time: booking.end_time,
//             created_at: booking.created_at,
//             updated_at: booking.updated_at,
//             deleted_at: booking.deleted_at,
//         })
//     }

//     pub async fn cancel_booking(&self, booking_id: i32) -> Result<(BookingEntity)> {
//         let pool = self.pool.clone();
//         let mut conn = pool.get()?;

//         let deleted_booking = bookings::table
//             .filter(bookings::id.eq(booking_id))
//             .first::<BookingEntity>(&mut conn)
//             .optional()?
//             .ok_or_else(|| anyhow::anyhow!("Room with id {} not found", booking_id))?;
//         Ok(deleted_booking)
//     }
// }
