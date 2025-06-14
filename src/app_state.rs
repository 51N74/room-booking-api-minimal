// src/app_state.rs

// หรือ path ที่ถูกต้องของ AdminService
use crate::application::booking_service::BookingService;
use crate::{application::{admin_service::AdminService, room_service::RoomService, user_service::UserService}, infrastructure::{database::DbPool, jwt::JwtService}};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub room_service: RoomService,
    pub user_service: UserService,
    pub admin_service: AdminService,
    pub jwt_service: JwtService,
    pub booking_service: BookingService,
    
    // pub booking_service: BookingService, // ถ้ามี
}