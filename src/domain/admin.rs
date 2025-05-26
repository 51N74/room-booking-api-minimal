use crate::infrastructure::schema::admins;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = admins)]
pub struct AdminEntity {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAdminEntity {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}
