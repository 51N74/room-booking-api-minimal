use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Duration, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // subject (user_id)
    pub role: String,     // "user" หรือ "admin"
    pub exp: usize,       // expiration time
    pub iat: usize,       // issued at
}

pub struct JwtService;

impl JwtService {
    // สร้าง JWT Token
    pub fn create_token(user_id: i32, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        
        let now = Utc::now();
        let expire = now + Duration::hours(24); // Token หมดอายุใน 24 ชั่วโมง
        
        let claims = Claims {
            sub: user_id.to_string(),
            role: role.to_string(),
            exp: expire.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    // ตรวจสอบ JWT Token
    pub fn verify_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
    }
}