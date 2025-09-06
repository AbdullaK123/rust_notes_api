use anyhow::Error;
use diesel_async::{AsyncPgConnection, RunQueryDsl}; // Use diesel_async::RunQueryDsl
use std::ops::DerefMut;
use uuid::Uuid;
use crate::config::{DbPool};
use anyhow::Result;
use crate::models::{User, UpdateUser, NewUser};

pub struct UserRepository {
    pub pool: DbPool
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    async fn get_connection(&self) -> Result<impl DerefMut<Target = AsyncPgConnection> + '_, Error> {
        self.pool.get()
            .await
            .map_err(|e| anyhow::anyhow!("Error getting connection: {}", e))
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let mut conn = self.get_connection().await?;
        let users: Vec<User> = diesel::sql_query(
            "SELECT
                id,
                username,
                email,
                password_hash,
                created_at,
                updated_at
             FROM users
             WHERE id = $1"
        )
            .bind::<diesel::sql_types::Uuid, _>(id)
            .load(&mut conn)
            .await?;

        Ok(users.into_iter().next())
    }
}