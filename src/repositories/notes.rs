use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use crate::models::{Note, NewNote, UpdateNote};

pub struct NoteRepository {
    pool: PgPool,
}

impl NoteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_note_by_id(&self, note_id: Uuid, user_id: Uuid) -> Result<Option<Note>> {
        let note = sqlx::query_as!(
            Note,
            r#"
            SELECT 
                id, 
                user_id, 
                title, 
                content, 
                created_at, 
                updated_at
            FROM notes 
            WHERE id = $1 AND user_id = $2
            "#,
            note_id,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(note)
    }

    pub async fn get_user_notes(&self, user_id: Uuid, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Note>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let notes = sqlx::query_as!(
            Note,
            r#"
            SELECT 
                id, 
                user_id, 
                title, 
                content, 
                created_at, 
                updated_at
            FROM notes
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(notes)
    }

    pub async fn create_note(&self, new_note: NewNote) -> Result<Note> {
        let note = sqlx::query_as!(
            Note,
            r#"
            INSERT INTO notes (user_id, title, content)
            VALUES ($1, $2, $3)
            RETURNING 
                id, 
                user_id, 
                title, 
                content, 
                created_at, 
                updated_at
            "#,
            new_note.user_id,
            new_note.title,
            new_note.content
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(note)
    }

    pub async fn update_note(&self, note_id: Uuid, user_id: Uuid, update_note: UpdateNote) -> Result<Option<Note>> {
        let note = sqlx::query_as!(
            Note,
            r#"
            UPDATE notes
            SET 
                title = COALESCE($3, title),
                content = COALESCE($4, content),
                updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            RETURNING 
                id, 
                user_id, 
                title, 
                content, 
                created_at, 
                updated_at
            "#,
            note_id,
            user_id,
            update_note.title,
            update_note.content
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(note)
    }

    pub async fn delete_note(&self, note_id: Uuid, user_id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM notes 
            WHERE id = $1 AND user_id = $2
            "#,
            note_id,
            user_id
        )
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn search_notes(&self, user_id: Uuid, query: &str, limit: Option<i64>) -> Result<Vec<Note>> {
        let limit = limit.unwrap_or(50);

        let notes = sqlx::query_as!(
            Note,
            r#"
            SELECT 
                id, 
                user_id, 
                title, 
                content, 
                created_at, 
                updated_at
            FROM notes
            WHERE 
                user_id = $1 
                AND to_tsvector('english', title || ' ' || content) @@ plainto_tsquery('english', $2)
            ORDER BY 
                ts_rank(to_tsvector('english', title || ' ' || content), plainto_tsquery('english', $2)) DESC
            LIMIT $3
            "#,
            user_id,
            query,
            limit
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(notes)
    }
}