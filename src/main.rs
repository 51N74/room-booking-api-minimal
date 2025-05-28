use std::env;

use anyhow::Result;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use room_booking_api_minimal::{application::{admin_service::AdminService, room_service::RoomService, user_service::UserService}, infrastructure::{admin_repository::AdminRepository, database::establish_connection_pool, room_repository::RoomRepository, user_repository::UserRepository}, presentation::{admin_handler::{login_admin_handler, register_admin_handler}, room_handler::add_room_handler, user_handler::{login_user_handler, register_user_handler}}};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // สร้าง DB Connection Pool
    let db_pool = establish_connection_pool();

    // สร้าง RoomService
    let room_repo = RoomRepository::new(db_pool.clone());
    let room_service = RoomService::new(room_repo);

    // สร้าง UserService
    let user_repo = UserRepository::new(db_pool.clone());
    let user_service = UserService::new(user_repo);

    // สร้าง AdminService
    let admin_repo = AdminRepository::new(db_pool.clone());
    let admin_service = AdminService::new(admin_repo);

    let app: Router = Router::new()
        .route("/rooms", post(add_room_handler))
        // .route("/rooms", get(get_all_rooms_handler))
        // .route("/rooms/:room_id", get(get_room_by_id_handler))
        // .route("/rooms/:room_id", patch(update_room_handler))
        // .route("/rooms/:room_id", delete(delete_room_handler))
        // .route("/rooms/by-status", get(get_rooms_by_status_handler))
        .with_state(room_service) // ใช้ room_service เป็น state สำหรับ route นี้
        .route("/register", post(register_user_handler))
        .route("/login/user", post(login_user_handler))
        .with_state(user_service)
        .route("/admin", post(register_admin_handler))
        .route("/login/admin", post(login_admin_handler))
        .with_state(admin_service);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://localhost:3000 ");
    axum::serve(listener, app).await?;
    Ok(())
}
