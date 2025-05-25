use std::env;

use anyhow::Result;
use axum::{routing::post, Router};
use room_booking_api_minimal::{application::room_service::RoomService, infrastructure::room_repository::RoomRepository, presentation::handler::create_room_handler};
use tokio::net::TcpListener;
#[tokio::main]
async fn main()->Result<()> {
    dotenv::dotenv().ok();
    
   let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let repo = RoomRepository::new(&database_url);
    let service = RoomService::new(repo);

    let app: Router = Router::new()
        .route("/rooms", post(create_room_handler))
        .with_state(service);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://localhost:3000 ");
    axum::serve(listener, app).await?;
    Ok(())

}
 