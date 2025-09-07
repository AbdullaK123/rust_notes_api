use actix_cors::Cors;
use actix_web::http::Method;
use crate::config::Settings;

pub fn create_cors_config(settings: &Settings) -> Cors {
    // Build CORS configuration properly
    let mut cors = Cors::default();
    
    // Add allowed origins
    for origin in &settings.cors.allowed_origins {
        cors = cors.allowed_origin(origin.as_str());
    }
    
    // Convert string methods to HTTP Method enum and add them
    let methods: Vec<Method> = settings.cors.allowed_methods
        .iter()
        .filter_map(|method_str| {
            match method_str.to_uppercase().as_str() {
                "GET" => Some(Method::GET),
                "POST" => Some(Method::POST),
                "PUT" => Some(Method::PUT),
                "DELETE" => Some(Method::DELETE),
                "PATCH" => Some(Method::PATCH),
                "HEAD" => Some(Method::HEAD),
                "OPTIONS" => Some(Method::OPTIONS),
                _ => {
                    eprintln!("Warning: Unknown HTTP method: {}", method_str);
                    None
                }
            }
        })
        .collect();
    
    cors = cors.allowed_methods(methods);
    
    // Add allowed headers
    for header in &settings.cors.allowed_headers {
        cors = cors.allowed_header(header.as_str());
    }
    
    // Add other CORS settings
    cors = cors.supports_credentials();
    
    if settings.cors.allow_credentials {
        cors = cors.supports_credentials();
    }
    
    cors
}