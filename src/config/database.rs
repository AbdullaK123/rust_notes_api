use diesel_async::{
    RunQueryDsl,
    pooled_connection::{bb8::{
        Pool, PooledConnection,
    }, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use anyhow::Result;
use std::time::Duration;
use crate::config::settings::DatabaseSettings;

pub type DbPool = Pool<AsyncPgConnection>;
pub type DbConnection = PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn test_connection(pool: &DbPool) -> Result<()> {
    let mut conn = pool.get().await
        .map_err(|e| anyhow::anyhow!("Failed to get connection from pool: {}", e))?;

    diesel::sql_query("SELECT 1")
        .execute(&mut conn)
        .await
        .map_err(|e| anyhow::anyhow!("Database test failed: {}", e))?;

    Ok(())
}

pub async fn create_pool(settings: &DatabaseSettings) -> Result<DbPool> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&settings.url);

    let pool = Pool::builder()
        .max_size(settings.max_connections)
        .min_idle(Some(settings.min_idle_connections))
        .connection_timeout(Duration::from_secs(settings.timeout))  // Added connection_timeout
        .max_lifetime(Some(Duration::from_secs(settings.max_lifetime)))
        .build(manager)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create database pool: {}", e))?;

    // Test the connection
    test_connection(&pool).await?;

    println!("✅ Database pool created successfully with {} max connections", settings.max_connections);

    Ok(pool)
}