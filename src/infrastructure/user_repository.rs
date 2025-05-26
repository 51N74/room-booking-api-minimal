use diesel::SqliteConnection;
use anyhow::Result;
use crate::domain::user::RegisterUserEntity;
use diesel::prelude::*;
use super::schema::users;

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>,
}

impl UserRepository {
    pub fn new(database_url: &str) -> Self {
        let manager = diesel::r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = diesel::r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool");
        UserRepository { pool }
    }

    pub async fn register_user(&self,user:RegisterUserEntity)->Result<RegisterUserEntity>{
        let new_user = (
            users::username.eq(user.username.clone()),
            users::password_hash.eq(user.password_hash.clone()),
        );

        let mut conn = self.pool.get()?;

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut conn)
            .map_err(|e| anyhow::anyhow!("Failed to insert order: {}", e))?;

       let id = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result::<i32>(&mut conn)
        .map_err(|e| anyhow::anyhow!("Failed to get last insert id: {}", e))?;


        Ok(RegisterUserEntity {
            id,
            username:user.username,
            password_hash: user.password_hash,
        })
    }


}
