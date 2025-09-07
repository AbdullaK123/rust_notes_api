use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ===== DATABASE MODELS =====

#[derive(Deserialize, Serialize, Clone, Debug, sqlx::FromRow)]
pub struct Note {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewNote {
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub search: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNoteDto {
    pub title: String,
    pub content: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserNotes {
    pub notes: Vec<Note>,
}

// ===== HELPER METHODS =====

impl NewNote {
    pub fn new(user_id: Uuid, title: String, content: String) -> Self {
        Self {
            user_id,
            title,
            content,
        }
    }
}

impl UpdateNote {
    pub fn new() -> Self {
        Self {
            title: None,
            content: None,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
}