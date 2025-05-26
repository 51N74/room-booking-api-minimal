use diesel::SqliteConnection;
use anyhow::Result;
use crate::domain::admin::RegisterAdminEntity;
use diesel::prelude::*;
use super::schema::admins;

#[derive(Debug, Clone)]
pub struct AdminRepository {
    pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>,
}

impl AdminRepository {
    pub fn new(database_url: &str) -> Self {
        let manager = diesel::r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = diesel::r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool");
        AdminRepository { pool }
    }

    pub async fn register_admin(&self,admin:RegisterAdminEntity)->Result<RegisterAdminEntity>{
        let new_user = (
            admins::username.eq(admin.username.clone()),
            admins::password_hash.eq(admin.password_hash.clone()),
        );

        let mut conn = self.pool.get()?;

        diesel::insert_into(admins::table)
            .values(&new_user)
            .execute(&mut conn)
            .map_err(|e| anyhow::anyhow!("Failed to insert order: {}", e))?;

       let id = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()",
        ))
        .get_result::<i32>(&mut conn)
        .map_err(|e| anyhow::anyhow!("Failed to get last insert id: {}", e))?;


        Ok(RegisterAdminEntity {
            id,
            username:admin.username,
            password_hash: admin.password_hash,
        })
    }


}
