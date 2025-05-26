use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::infrastructure::schema::users;

#[derive(Debug, Clone, Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UserEntity{
    pub id:i32,
    pub username:String,
    pub password_hash:String,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO{
    pub username:String,
    pub password_hash:String,

    
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUserEntity{
    pub id:i32,
    pub username:String,
    pub password_hash:String,
}