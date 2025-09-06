use crate::config::settings::RedisSettings;
use anyhow::Result;
use actix_session::storage::RedisSessionStore;

pub async fn create_redis_session_store(settings: &RedisSettings) -> Result<RedisSessionStore> {
    let store = RedisSessionStore::new(&settings.url)
        .await
        .map_err(
            |e| anyhow::anyhow!("Failed to create redis session store: {}", e)
        )?;

    Ok(store)
}