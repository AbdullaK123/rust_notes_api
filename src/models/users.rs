use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::future::{ready, Ready};
use crate::utils::hash_password;
use actix_web::dev::Payload;
// ===== DATABASE MODELS ======

#[derive(Serialize, Clone, Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: Uuid,
    pub is_active: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserId (pub Option<Uuid>);

impl FromRequest for UserId {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // Look for the UserId in extensions (inserted by middleware)
        match req.extensions().get::<UserId>() {
            Some(user_id) => ready(Ok(user_id.clone())),
            None => {
                // This should never happen if auth middleware is working correctly
                ready(Err(actix_web::error::ErrorUnauthorized("Missing user ID")))
            }
        }
    }
}

#[derive(Clone)]
pub struct AuthenticatedUser(pub Uuid);

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.extensions().get::<UserId>() {
            Some(UserId(Some(user_id))) => ready(Ok(AuthenticatedUser(*user_id))),
            _ => ready(Err(actix_web::error::ErrorUnauthorized("Authentication required")))
        }
    }
}



// ===== HELPER METHODS =====

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }   
}

impl TryFrom<RegistrationRequest> for NewUser {
    type Error = String;
    fn try_from(value: RegistrationRequest) -> Result<Self, Self::Error> {

        // Hash the password
        let password_hash = hash_password(&value.password)
            .map_err(|_| "Failed to hash password".to_string())?;

        Ok(NewUser {
            username: value.username.trim().to_string(),
            email: value.email.to_lowercase(),
            password_hash,
        })
    }
}

impl From<User> for UserSession {
    fn from(user: User) -> Self {
        Self {
            user_id: user.id,
            is_active: true
        }
    }
}

impl NewUser {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        Self {
            username,
            email,
            password_hash,
        }
    }
}

impl UpdateUser {
    pub fn new() -> Self {
        Self {
            username: None,
            email: None,
            password_hash: None,
        }
    }

    pub fn with_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn with_password_hash(mut self, password_hash: String) -> Self {
        self.password_hash = Some(password_hash);
        self
    }
}