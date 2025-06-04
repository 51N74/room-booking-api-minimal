// src/middleware/auth.rs

use std::sync::Arc;

use axum::{
    body::Body, extract::{Request}, http::{header,StatusCode}, middleware::Next, response::{IntoResponse, Response}, Extension, Json
};

use serde_json::json;
use crate::app_state::AppState; // เพื่อเข้าถึง Secret หรืออื่นๆ ถ้าจำเป็น

// Middleware สำหรับตรวจสอบ Token จาก Cookie (สำหรับผู้ใช้ทั่วไป)
pub async fn auth_middleware(
    // jar: CookieJar, // <--- NOTE: หากไม่ใช้ Cookie แล้ว สามารถลบ CookieJar ออกไปได้
    // เราจะดึง Token จาก Authorization header แทน
    Extension(state): Extension<Arc<AppState>>,
    mut request: Request<Body>, // รับ Request<Body>
    next: Next,
) -> Result<Response<Body>, Response<Body>> { // คืนค่า Response<Body>
    eprintln!("\n--- DEBUG: auth_middleware entered ---");
    // 1. ดึง Token จาก Authorization Header (Bearer Token)
    let auth_header = request.headers().get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

     let token = if let Some(header_value) = auth_header {
        eprintln!("DEBUG: Authorization header found: '{}'", header_value); // <--- Debug Point 2
        if header_value.starts_with("Bearer ") {
            let extracted_token = header_value[7..].to_owned();
            eprintln!("DEBUG: Extracted token: '{}'", extracted_token); // <--- Debug Point 3
            Some(extracted_token)
        } else {
            eprintln!("DEBUG: Authorization header found, but missing 'Bearer ' prefix."); // <--- Debug Point 4
            None
        }
    } else {
        eprintln!("DEBUG: Authorization header NOT found."); // <--- Debug Point 5
        None
    };

    if let Some(token) = token {
        // 2. Decode และ Validate Token ด้วย JwtService
        // *** แก้ไข: ใช้ app_state.jwt_service.decode_token() ***
        match state.jwt_service.decode_token(&token) {
            Ok(claims) => {
                // 3. ตรวจสอบ Role (ถ้าจำเป็นสำหรับ Middleware นี้)
                // Middleware นี้จะใช้สำหรับ User ทั่วไปเข้าถึง resource
                 eprintln!("DEBUG: Token decoded successfully. Claims: {:?}", claims);
                if claims.role != "user" {
                    eprintln!("DEBUG: Role mismatch. Expected 'user', got '{}'.", claims.role);
                    let forbidden_response = (
                        StatusCode::FORBIDDEN,
                        Json(json!({"error": "Forbidden: Insufficient permissions."})),
                    ).into_response(); // into_response() ก็พอ
                    return Err(forbidden_response.into_response()); // map_into_response เพื่อแปลง Body Type
                }
                eprintln!("DEBUG: Role 'user' confirmed. Proceeding to next handler.");
                // 4. เก็บ Claims ลงใน Request Extensions
                request.extensions_mut().insert(claims);
                // 5. ส่ง Request ต่อไปยัง Handler ถัดไป
                Ok(next.run(request).await)
            },
            Err(e) => {
                eprintln!("ERROR: Token decode failed: {:?}", e);
                // Token ไม่ถูกต้อง/หมดอายุ
                
                let unauthorized_response = (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": format!("Unauthorized: Invalid token. {}", e)})),
                ).into_response();
                Err(unauthorized_response.into_response()) // map_into_response เพื่อแปลง Body Type
            }
        }
    } else {
        eprintln!("ERROR: No token found in Authorization header. Returning 401."); 
        // ไม่มี Token ใน Header
        let unauthorized_response = (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Unauthorized: Authorization token missing or malformed."})),
        ).into_response();
        Err(unauthorized_response.into_response())
    }
}

// Middleware สำหรับ Admin (คล้ายกัน แต่ตรวจสอบ role = "admin")
pub async fn admin_middleware(
    Extension(state): Extension<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, Response<Body>> {
    eprintln!("\n--- DEBUG: auth_middleware entered ---"); 
    let auth_header = request.headers().get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = if let Some(header_value) = auth_header {
        eprintln!("DEBUG: Authorization header found: '{}'", header_value); // <--- Debug Point 2
        if header_value.starts_with("Bearer ") {
            let extracted_token = header_value[7..].to_owned();
            eprintln!("DEBUG: Extracted token: '{}'", extracted_token); // <--- Debug Point 3
            Some(extracted_token)
        } else {
            eprintln!("DEBUG: Authorization header found, but missing 'Bearer ' prefix."); // <--- Debug Point 4
            None
        }
    } else {
        eprintln!("DEBUG: Authorization header NOT found."); // <--- Debug Point 5
        None
    };


    if let Some(token) = token {
        match state.jwt_service.decode_token(&token) { // *** ใช้ decode_token ***
            Ok(claims) => {
                eprintln!("DEBUG: Token decoded successfully. Claims: {:?}", claims);
                if claims.role != "admin" { // ตรวจสอบ role เป็น "admin"
                eprintln!("DEBUG: Role mismatch. Expected 'user', got '{}'.", claims.role);
                    let forbidden_response = (
                        StatusCode::FORBIDDEN,
                        Json(json!({"error": "Forbidden: Insufficient permissions."})),
                    ).into_response();
                    return Err(forbidden_response.into_response());
                }
                eprintln!("DEBUG: Role 'user' confirmed. Proceeding to next handler.");
                request.extensions_mut().insert(claims);
                Ok(next.run(request).await)
            },
            Err(e) => {
                eprintln!("ERROR: Token decode failed: {:?}", e);
                let unauthorized_response = (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": format!("Unauthorized: Invalid token. {}", e)})),
                ).into_response();
                Err(unauthorized_response.into_response())
            }
        }
    } else {
        eprintln!("ERROR: No token found in Authorization header. Returning 401.");
        let unauthorized_response = (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Unauthorized: Authorization token missing or malformed."})),
        ).into_response();
        Err(unauthorized_response.into_response())
    }

}