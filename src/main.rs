use anyhow::Result;
use axum::{
    Router,

    middleware,
    routing::{delete, get, patch, post},
};
use room_booking_api_minimal::{app_state::AppState, application::booking_service::BookingService, infrastructure::{jwt::JwtService}};

use room_booking_api_minimal::{
    application::{
        admin_service::AdminService, room_service::RoomService, user_service::UserService,
    },
    infrastructure::{
        admin_repository::AdminRepository,database::establish_connection_pool,
        room_repository::RoomRepository, user_repository::UserRepository,
    },
    middleware::auth::{admin_middleware, auth_middleware},
    presentation::{
        admin_handler::{login_admin_handler, register_admin_handler},
        booking_handler::{
            cancel_booking_handler, create_booking_handler, get_all_bookings_handler,
            get_user_bookings_handler,
        },
        room_handler::{
            add_room_handler, delete_room_handler, get_all_active_rooms_handler,
            get_all_room_handler, get_room_by_id_handler, update_room_handler,
        },
        test_handler::{test_protected_admin_route, test_protected_user_route},
        user_handler::{login_user_handler, register_user_handler},
    },
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    // ดึง Secret Key จาก Environment Variable
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");
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

    

    // *** สร้าง BookingService โดยส่ง db_pool เข้าไปโดยตรง ***
    // (BookingRepository ไม่จำเป็นต้องสร้างตรงนี้แล้ว)
    let booking_service = BookingService::new(db_pool.clone());

    // *** สร้าง JwtService instance ***
    let jwt_service = JwtService::new(&jwt_secret);

    // *** สร้าง AppState instance (ตัวแปร app_state ตัวเล็ก) ***
    let app_state = AppState {
        db_pool: db_pool.clone(),
        room_service: room_service.clone(),
        user_service: user_service.clone(),
        admin_service: admin_service.clone(),
        booking_service: booking_service.clone(),
        jwt_service: jwt_service.clone(),
    };

    let app: Router = Router::new()
        // User Login/Register (ไม่ต้องมี Middleware)
        // User Login/Register routes
        .route("/register", post(register_user_handler))
        .route("/login/user", post(login_user_handler))
        // Admin Login/Register routes
        .route("/admin", post(register_admin_handler))
        .route("/login/admin", post(login_admin_handler))
        // *** Router สำหรับเส้นทางที่ Admin เท่านั้นที่เข้าถึงได้ ***
        .nest(
            "/admin",
            Router::new()
                .route("/rooms", post(add_room_handler))
                .route("/rooms/:room_id", patch(update_room_handler))
                .route("/rooms/:room_id", delete(delete_room_handler))
                .route("/bookings", get(get_all_bookings_handler))
                // *** เพิ่ม route ทดสอบสำหรับ admin ***
                .route("/test-admin", get(test_protected_admin_route))
                // *** ใช้ตัวแปร app_state (ตัวเล็ก) ที่นี่ ***
                .with_state(app_state.clone()) // <-- ถูกต้องแล้ว
                .layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    admin_middleware,
                )),
        )
        // *** Router สำหรับเส้นทางที่ User ทั่วไป (ต้อง Login) เข้าถึงได้ ***
        .nest(
            "/bookings",
            Router::new()
                .route("/", post(create_booking_handler))
                .route("/:id", delete(cancel_booking_handler))
                .route("/user", get(get_user_bookings_handler))
                // *** เพิ่ม route ทดสอบสำหรับ user ***
                .route("/test-user", get(test_protected_user_route))
                // *** ใช้ตัวแปร app_state (ตัวเล็ก) ที่นี่ ***
                .with_state(app_state.clone()) // <-- ถูกต้องแล้ว
                .layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    auth_middleware,
                )),
        )
        // *** Router สำหรับเส้นทาง Public หรือที่ User ทั่วไปเข้าถึงได้โดยไม่ต้อง Login/Admin ***
        .route("/rooms/active", get(get_all_active_rooms_handler))
        .route("/rooms", get(get_all_room_handler))
        .route("/rooms/:room_id", get(get_room_by_id_handler))
        // *** ใช้ตัวแปร app_state (ตัวเล็ก) ที่ Router หลักด้วย ***
        .with_state(app_state.clone()); // <-- ถูกต้องแล้ว

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://localhost:3000 ");
    axum::serve(listener, app).await?;
    Ok(())
}
