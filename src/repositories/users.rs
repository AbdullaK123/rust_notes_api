use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use crate::models::{User, NewUser, UpdateUser};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, 
                username, 
                email, 
                password_hash, 
                created_at, 
                updated_at
            FROM users 
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, 
                username, 
                email, 
                password_hash, 
                created_at, 
                updated_at
            FROM users 
            WHERE email = $1
            "#,
            email
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, 
                username, 
                email, 
                password_hash, 
                created_at, 
                updated_at
            FROM users 
            WHERE username = $1
            "#,
            username
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING 
                id, 
                username, 
                email, 
                password_hash, 
                created_at, 
                updated_at
            "#,
            new_user.username,
            new_user.email,
            new_user.password_hash
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn update_user(&self, user_id: Uuid, update_user: UpdateUser) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET 
                username = COALESCE($2, username),
                email = COALESCE($3, email),
                password_hash = COALESCE($4, password_hash),
                updated_at = NOW()
            WHERE id = $1
            RETURNING 
                id, 
                username, 
                email, 
                password_hash, 
                created_at, 
                updated_at
            "#,
            user_id,
            update_user.username,
            update_user.email,
            update_user.password_hash
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users 
            WHERE id = $1
            "#,
            user_id
        )
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}