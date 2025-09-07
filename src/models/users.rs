use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::utils::hash_password;
// ===== DATABASE MODELS =====

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

pub struct UserId (pub Option<Uuid>);


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