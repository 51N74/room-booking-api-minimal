use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
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

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation, // เพิ่ม field สำหรับ Validation
}

impl JwtService {
     pub fn new(secret: &str) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());

        let mut validation = Validation::new(Algorithm::HS512); // ใช้ Algorithm เดียวกันกับตอน encode
        validation.validate_exp = true; // ตรวจสอบวันหมดอายุ (ควรเป็น true)
        validation.leeway = 60; // อนุญาตให้มีเวลาเหลื่อมได้ 60 วินาที

        JwtService {
            encoding_key,
            decoding_key,
            validation,
        }
    }

    // *** แก้ไข: เพิ่ม &self และใช้ self.encoding_key ***
    pub fn create_token(&self, user_id: i32, role: &str) -> Result<String, String> {
        let now = Utc::now();
        let expires_in = Duration::days(1); // 24 ชั่วโมง
        let exp = (now + expires_in).timestamp();
        let iat = now.timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            role: role.to_string(),
            exp: exp as usize,
            iat: iat as usize,
        };

        let header = Header::new(Algorithm::HS512);
        // *** ใช้ &self.encoding_key แทน Self::get_secret() ***
        encode(&header, &claims, &self.encoding_key)
            .map_err(|e| format!("Failed to create token: {}", e))
    }

   // *** แก้ไข: เพิ่ม &self และใช้ self.decoding_key กับ self.validation ***
    pub fn decode_token(&self, token: &str) -> Result<Claims, String> {
        // *** ใช้ &self.decoding_key และ &self.validation แทนการสร้างใหม่จาก env variable ***
        decode::<Claims>(
            token,
            &self.decoding_key,
            &self.validation,
        )
        .map(|data| data.claims)
        .map_err(|e| format!("Invalid token: {}", e))
    }
}
