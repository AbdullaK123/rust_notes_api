use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;
use std::time::Duration;
use crate::config::settings::DatabaseSettings;

pub type DbPool = PgPool;

pub async fn create_pool(settings: &DatabaseSettings) -> Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .min_connections(settings.min_idle_connections)
        .acquire_timeout(Duration::from_secs(settings.timeout))
        .idle_timeout(Duration::from_secs(settings.max_lifetime))
        .max_lifetime(Duration::from_secs(settings.max_lifetime))
        .connect(&settings.url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create database pool: {}", e))?;

    // Test connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database test failed: {}", e))?;

    println!("✅ Database pool created successfully with {} max connections", settings.max_connections);

    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;

    println!("✅ Database migrations completed");
    Ok(())
}