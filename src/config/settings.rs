use serde::{Deserialize, Deserializer};
use anyhow::Result;
use dotenvy;

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
    #[serde(default = "default_max_db_connections")]
    pub max_connections: u32,
    #[serde(default = "default_min_db_idle_connections")]
    pub min_idle_connections: u32,
    #[serde(default = "default_db_timeout")]
    pub timeout: u64,
    #[serde(default = "default_db_max_lifetime")]
    pub max_lifetime: u64,
    #[serde(default = "default_db_migration_auto")]
    pub auto_migrate: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RedisSettings {
    pub url: String,
    #[serde(default = "default_redis_pool_size")]
    pub pool_size: u32,
    #[serde(default = "default_redis_timeout")]
    pub timeout: u64,
    #[serde(default = "default_redis_max_retries")]
    pub max_retries: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CookieSettings {
    #[serde(default = "default_cookie_name")]
    pub name: String,
    #[serde(default = "default_cookie_http_only")]
    pub http_only: bool,
    #[serde(default = "default_cookie_secure")]
    pub secure: bool,
    #[serde(default = "default_cookie_same_site")]
    pub same_site: String,
    #[serde(default = "default_cookie_domain")]
    pub domain: String,
    #[serde(default = "default_cookie_max_age")]
    pub max_age: Option<u32>,
    #[serde(default = "default_cookie_path")]
    pub path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CorsSettings {
    #[serde(default = "default_allowed_origins", deserialize_with = "deserialize_comma_separated")]
    pub allowed_origins: Vec<String>,
    #[serde(default = "default_allowed_methods", deserialize_with = "deserialize_comma_separated")]
    pub allowed_methods: Vec<String>,
    #[serde(default = "default_allowed_headers", deserialize_with = "deserialize_comma_separated")]
    pub allowed_headers: Vec<String>,
    #[serde(default = "default_allow_credentials")]
    pub allow_credentials: bool,
    #[serde(default = "default_cors_max_age")]
    pub max_age: u32,
    #[serde(default = "default_expose_headers", deserialize_with = "deserialize_comma_separated")]
    pub expose_headers: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ApiSettings {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_request_timeout")]
    pub request_timeout: u64,
    #[serde(default = "default_max_request_size")]
    pub max_request_size: u32,
    #[serde(default = "default_rate_limit")]
    pub rate_limit: Option<u32>,
    #[serde(default = "default_api_prefix")]
    pub api_prefix: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Settings {
    pub secret_key: String,
    #[serde(default = "default_environment")]
    pub environment: Environment,
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub api: ApiSettings,
    pub cors: CorsSettings,
    pub cookie: CookieSettings,
}

// Additional default functions for new fields
fn default_db_timeout() -> u64 {
    30 // 30 seconds
}

fn default_db_idle_timeout() -> u64 {
    300 // 5 minutes
}

fn default_db_max_lifetime() -> u64 {
    1800 // 30 minutes
}

fn default_db_migration_auto() -> bool {
    false
}


fn default_redis_timeout() -> u64 {
    30 // 30 seconds
}

fn default_redis_max_retries() -> u32 {
    3
}

fn default_cookie_http_only() -> bool {
    true
}

fn default_cookie_max_age() -> Option<u32> {
    Some(86400) // 24 hours in seconds
}

fn default_cookie_path() -> String {
    "/".to_string()
}

fn default_expose_headers() -> Vec<String> {
    vec![]
}

fn default_request_timeout() -> u64 {
    30 // 30 seconds
}

fn default_max_request_size() -> u32 {
    16777216 // 16MB in bytes
}

fn default_rate_limit() -> Option<u32> {
    Some(1000) // 1000 requests per minute
}

fn default_api_prefix() -> String {
    "/api".to_string()
}

fn default_max_db_connections() -> u32 {10}
fn default_min_db_idle_connections() -> u32 {2}

fn default_port() -> u16 { 8080 }
fn default_host() -> String { "0.0.0.0".to_string()}

fn default_allowed_origins() -> Vec<String> {
    vec!["http://localhost:3000".to_string()]
}

fn default_allowed_methods() -> Vec<String> {
    vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()]
}

fn default_allowed_headers() -> Vec<String> {
    vec!["Content-Type".to_string(), "Authorization".to_string()]
}

fn default_allow_credentials() -> bool {
    true
}

fn default_cors_max_age() -> u32 {
    86400
}

fn default_cookie_name() -> String {
    "session".to_string()
}

fn default_cookie_secure() -> bool {
    false
}

fn default_cookie_same_site() -> String {
    "Lax".to_string()
}

fn default_cookie_domain() -> String {
    "localhost".to_string()
}

fn default_environment() -> Environment {
    Environment::Development
}

fn default_redis_pool_size() -> u32 {
    10
}

impl Settings {
    pub fn new() -> Result<Self> {
        dotenvy::dotenv().ok();
        let settings: Settings = serde_env::from_env().map_err(|e| {
            anyhow::anyhow!("Failed to load settings: {}", e)
        })?;

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