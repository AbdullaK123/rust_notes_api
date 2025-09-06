use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::schema::users;

// ===== DATABASE MODELS =====

#[derive(Queryable, Selectable, QueryableByName, Serialize, Clone, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>
}

// ===== HELPER METHODS =====

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