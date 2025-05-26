use std::env;

use anyhow::Result;
use axum::{routing::{get, post}, Router};
use room_booking_api_minimal::{application::{admin_service::AdminService, room_service::RoomService, user_service::UserService}, infrastructure::{admin_repository::AdminRepository, room_repository::RoomRepository, user_repository::UserRepository}, presentation::{admin_handler::create_admin_handler, room_handler::{create_room_handler, get_all_rooms_handler, get_room_by_id_handler}, user_handler::create_user_handler}};
use tokio::net::TcpListener;
#[tokio::main]
async fn main()->Result<()> {
    dotenv::dotenv().ok();
    
   let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

   // สร้าง RoomService
    let room_repo = RoomRepository::new(&database_url);
    let room_service = RoomService::new(room_repo);


    // สร้าง UserService
    let user_repo = UserRepository::new(&database_url);
    let user_service = UserService::new(user_repo);

    // สร้าง AdminService
    let admin_repo = AdminRepository::new(&database_url);
    let admin_service = AdminService::new(admin_repo);

    let app: Router = Router::new()
        .route("/rooms", post(create_room_handler))
        .route("/rooms",get(get_all_rooms_handler))
        .route("/rooms/:room_id", get(get_room_by_id_handler))
        .with_state(room_service) // ใช้ room_service เป็น state สำหรับ route นี้
        .route("/users", post(create_user_handler))
        .with_state(user_service)// ใช้ user_service เป็น state สำหรับ route นี้
        .route("/admin", post(create_admin_handler))
        .with_state(admin_service);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://localhost:3000 ");
    axum::serve(listener, app).await?;
    Ok(())

}
 