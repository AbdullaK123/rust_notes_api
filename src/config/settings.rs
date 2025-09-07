use serde::{Deserialize, Deserializer};
use anyhow::Result;
use dotenvy;
use std::env;

// Custom deserializer for comma-separated strings to Vec<String>
fn deserialize_comma_separated<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    pub min_idle_connections: u32,
    pub timeout: u64,
    pub max_lifetime: u64,
    pub auto_migrate: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RedisSettings {
    pub url: String,
    pub pool_size: u32,
    pub timeout: u64,
    pub max_retries: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CookieSettings {
    pub name: String,
    pub http_only: bool,
    pub secure: bool,
    pub same_site: String,
    pub domain: String,
    pub max_age: Option<u32>,
    pub path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CorsSettings {
    #[serde(deserialize_with = "deserialize_comma_separated")]
    pub allowed_origins: Vec<String>,
    #[serde(deserialize_with = "deserialize_comma_separated")]
    pub allowed_methods: Vec<String>,
    #[serde(deserialize_with = "deserialize_comma_separated")]
    pub allowed_headers: Vec<String>,
    pub allow_credentials: bool,
    pub max_age: u32,
    #[serde(deserialize_with = "deserialize_comma_separated")]
    pub expose_headers: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ApiSettings {
    pub port: u16,
    pub host: String,
    pub request_timeout: u64,
    pub max_request_size: u32,
    pub rate_limit: Option<u32>,
    pub api_prefix: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Settings {
    pub secret_key: String,
    pub environment: Environment,
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub api: ApiSettings,
    pub cors: CorsSettings,
    pub cookie: CookieSettings,
}

impl Settings {
    pub fn new() -> Result<Self> {
        dotenvy::dotenv().ok();

        let settings = Settings {
            secret_key: env::var("SECRET_KEY")
                .map_err(|_| anyhow::anyhow!("SECRET_KEY must be set"))?,
            
            environment: match env::var("ENVIRONMENT").unwrap_or_default().to_lowercase().as_str() {
                "production" => Environment::Production,
                _ => Environment::Development,
            },

            database: DatabaseSettings {
                url: env::var("DATABASE_URL")
                    .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?,
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
                min_idle_connections: env::var("DB_MIN_IDLE_CONNECTIONS")
                    .unwrap_or_else(|_| "2".to_string())
                    .parse()
                    .unwrap_or(2),
                timeout: env::var("DB_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
                max_lifetime: env::var("DB_MAX_LIFETIME")
                    .unwrap_or_else(|_| "1800".to_string())
                    .parse()
                    .unwrap_or(1800),
                auto_migrate: env::var("DB_AUTO_MIGRATE")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
            },

            redis: RedisSettings {
                url: env::var("REDIS_URL")
                    .map_err(|_| anyhow::anyhow!("REDIS_URL must be set"))?,
                pool_size: env::var("REDIS_POOL_SIZE")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
                timeout: env::var("REDIS_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
                max_retries: env::var("REDIS_MAX_RETRIES")
                    .unwrap_or_else(|_| "3".to_string())
                    .parse()
                    .unwrap_or(3),
            },

            api: ApiSettings {
                port: env::var("PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
                host: env::var("HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                request_timeout: env::var("API_REQUEST_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
                max_request_size: env::var("API_MAX_REQUEST_SIZE")
                    .unwrap_or_else(|_| "16777216".to_string())
                    .parse()
                    .unwrap_or(16777216),
                rate_limit: env::var("API_RATE_LIMIT")
                    .ok()
                    .and_then(|s| s.parse().ok()),
                api_prefix: env::var("API_PREFIX")
                    .unwrap_or_else(|_| "/api".to_string()),
            },

            cors: CorsSettings {
                allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "http://localhost:3000".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
                allowed_methods: env::var("CORS_ALLOWED_METHODS")
                    .unwrap_or_else(|_| "GET,POST,PUT,DELETE".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
                allowed_headers: env::var("CORS_ALLOWED_HEADERS")
                    .unwrap_or_else(|_| "Content-Type,Authorization".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
                allow_credentials: env::var("CORS_ALLOW_CREDENTIALS")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                max_age: env::var("CORS_MAX_AGE")
                    .unwrap_or_else(|_| "86400".to_string())
                    .parse()
                    .unwrap_or(86400),
                expose_headers: env::var("CORS_EXPOSE_HEADERS")
                    .unwrap_or_else(|_| "".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
            },

            cookie: CookieSettings {
                name: env::var("COOKIE_NAME")
                    .unwrap_or_else(|_| "session".to_string()),
                http_only: env::var("COOKIE_HTTP_ONLY")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                secure: env::var("COOKIE_SECURE")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
                same_site: env::var("COOKIE_SAME_SITE")
                    .unwrap_or_else(|_| "Lax".to_string()),
                domain: env::var("COOKIE_DOMAIN")
                    .unwrap_or_else(|_| "localhost".to_string()),
                max_age: env::var("COOKIE_MAX_AGE")
                    .ok()
                    .and_then(|s| s.parse().ok()),
                path: env::var("COOKIE_PATH")
                    .unwrap_or_else(|_| "/".to_string()),
            },
        };

        settings.validate()?;
        Ok(settings)
    }

    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }

    pub fn is_development(&self) -> bool {
        self.environment == Environment::Development
    }

    pub fn validate(&self) -> Result<()> {
        if self.secret_key.len() < 32 {
            return Err(anyhow::anyhow!("Secret key must be at least 32 characters long"));
        }
        if self.database.url.is_empty() {
            return Err(anyhow::anyhow!("Database URL must be set"));
        }
        if self.redis.url.is_empty() {
            return Err(anyhow::anyhow!("Redis URL must be set"));
        }
        if self.database.min_idle_connections > self.database.max_connections {
            return Err(anyhow::anyhow!("Min db idle connections must be less than max db connections"));
        }

        // Validate CORS origins are valid URLs (basic check)
        for origin in &self.cors.allowed_origins {
            if !origin.starts_with("http://") && !origin.starts_with("https://") {
                return Err(anyhow::anyhow!("Invalid CORS origin: {}", origin));
            }
        }

        // Validate API prefix starts with /
        if !self.api.api_prefix.starts_with('/') {
            return Err(anyhow::anyhow!("API prefix must start with /"));
        }

        Ok(())
    }
}

pub fn get_settings() -> Result<Settings> {
    Settings::new()
}