mod config;
mod models;
mod repositories;
mod controllers;
mod middleware;
mod services;
mod utils;

use actix_web::{web, App, HttpServer};
use config::{create_pool, create_redis_session_store, run_migrations, Settings};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Load settings
    let settings = Settings::new()?;

    // Create database pool
    let db_pool = create_pool(&settings.database).await?;

    // Run migrations
    run_migrations(&db_pool).await?;

    // Create Redis session store
    let redis_store = create_redis_session_store(&settings.redis).await?;

    println!("🚀 Starting server on {}:{}", settings.api.host, settings.api.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
        // TODO: Add your routes here
    })
        .bind((settings.api.host.as_str(), settings.api.port))?
        .run()
        .await?;

    Ok(())
}
