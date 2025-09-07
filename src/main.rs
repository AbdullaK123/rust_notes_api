mod config;
mod models;
mod repositories;
mod controllers;
mod middleware;
mod services;
mod utils;

use env_logger::{
    Env,
    init_from_env
};
use actix_session::SessionMiddleware;
use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use config::{create_pool, create_redis_session_store, run_migrations, Settings};
use crate::controllers::{configure_auth_controller, configure_notes_controller};
use crate::services::{UserService, NoteService};

// Health check endpoint
async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "rust_notes_api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now()
    })))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load settings
    let settings = Settings::new()?;

    // Create database pool and services
    let db_pool = create_pool(&settings.database).await?;
    let user_service = web::Data::new(UserService::new(db_pool.clone()));
    let note_service = web::Data::new(NoteService::new(db_pool.clone()));

    // Run migrations
    run_migrations(&db_pool).await?;

    // init logging
    init_from_env(Env::default().default_filter_or("info"));

    // Create Redis session store outside the closure
    let redis_store = create_redis_session_store(&settings.redis).await?;
    let secret_key = Key::from(settings.secret_key.as_bytes());
    
    println!("🚀 Starting server on {}:{}", settings.api.host, settings.api.port);

    HttpServer::new(move || {
        // Create session middleware with the pre-created store
        let session_middleware = SessionMiddleware::new(
            redis_store.clone(),
            secret_key.clone()
        );

        App::new()
            .app_data(user_service.clone())
            .app_data(note_service.clone())
            .wrap(Logger::default())
            .wrap(session_middleware)
            // Health check endpoint (no authentication required)
            .route("/health", web::get().to(health))
            .configure(configure_auth_controller)
            .configure(configure_notes_controller)
    })
        .bind((settings.api.host.as_str(), settings.api.port))?
        .run()
        .await?;

    Ok(())
}