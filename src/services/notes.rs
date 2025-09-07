use actix_web::{HttpResponse, Error};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{NewNote, UpdateNote, UserNotes};
use crate::repositories::NoteRepository;

pub struct NoteService {
    pub repo: NoteRepository
}

impl NoteService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: NoteRepository::new(pool)
        }
    }

    pub async fn get_note_by_id(
        &self,
        user_id: Uuid,
        note_id: Uuid
    ) -> Result<HttpResponse, Error> {
        let note = self.repo
            .get_note_by_id(note_id, user_id)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        match note {
            Some(extracted_note) => Ok(HttpResponse::Ok().json(extracted_note)),
            None => Ok(HttpResponse::NotFound().json(json!({ "message": "Note not found" })))
        }
    }

    pub async fn get_users_notes(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>
    ) -> Result<HttpResponse, Error> {
        let user_notes = self.repo
            .get_user_notes(user_id, limit, offset)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        
        Ok(HttpResponse::Ok().json(UserNotes { notes: user_notes }))
    }

    pub async fn search_notes(
        &self,
        user_id: Uuid,
        search_term: String,
        limit: Option<i64>
    ) -> Result<HttpResponse, Error> {
        let search_results = self.repo
            .search_notes(user_id, &search_term, limit)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        
        Ok(HttpResponse::Ok().json(search_results))
    }

    pub async fn create_note(
        &self,
        new_note: NewNote
    ) -> Result<HttpResponse, Error> {
        let new_note = self.repo
            .create_note(new_note)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        
        Ok(HttpResponse::Created().json(new_note))
    }

    pub async fn update_note(
        &self,
        user_id: Uuid,
        note_id: Uuid,
        updated_note: UpdateNote
    ) -> Result<HttpResponse, Error> {
        let note = self.repo
            .get_note_by_id(note_id, user_id)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(_) = note {
            let updated_note = self.repo
                .update_note(note_id, user_id, updated_note)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            match updated_note {
                Some(note) => {
                    Ok(HttpResponse::Ok().json(note))
                }
                None => {
                    Ok(HttpResponse::NotFound().json(json!({ "message": "Note not found" })))
                }
            }
        } else {
            Ok(HttpResponse::NotFound().json(json!({ "message": "Note not found" })))
        }
    }

    pub async fn delete_note(
        &self,
        user_id: Uuid,
        note_id: Uuid
    ) -> Result<HttpResponse, Error> {
        let deleted = self.repo
            .delete_note(note_id, user_id)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        
        if deleted {
            Ok(HttpResponse::NoContent().json(json!({ "message": "Note deleted" })))
        } else {
            Ok(HttpResponse::NotFound().json(json!({ "message": "Note not found" })))
        }
    }
}