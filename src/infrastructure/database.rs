use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection; // <<-- ใช้ SqliteConnection
use std::env;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url); // <<-- ใช้ SqliteConnection
    Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.")
}