use actix_web::{get, post, put, delete, web, HttpResponse, Error};
use actix_web::middleware::from_fn;
use uuid::Uuid;
use crate::middleware::auth_middleware;
use crate::models::{CreateNoteDto, QueryParams, UpdateNote, NewNote, AuthenticatedUser};
use crate::services::NoteService;

#[get("")]
async fn get_notes(
    user: AuthenticatedUser,
    query: web::Query<QueryParams>,
    service: web::Data<NoteService>
) -> Result<HttpResponse, Error> {
    if let Some(search_term) = &query.search {
        service.search_notes(user.0, search_term.clone(), query.limit).await
    } else {
        service.get_users_notes(user.0, query.limit, query.offset).await
    }
}

#[get("/{note_id}")]
async fn get_note(
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    service: web::Data<NoteService>
) -> Result<HttpResponse, Error> {
    let note_id = path.into_inner();
    service.get_note_by_id(user.0, note_id).await
}

#[post("")]
async fn create_note(
    user: AuthenticatedUser,
    payload: web::Json<CreateNoteDto>,
    service: web::Data<NoteService>
) -> Result<HttpResponse, Error> {
    let new_note = NewNote::new(user.0, payload.title.clone(), payload.content.clone());
    service.create_note(new_note).await
}

#[put("/{note_id}")]
async fn update_note(
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateNote>,
    service: web::Data<NoteService>
) -> Result<HttpResponse, Error> {
    let note_id = path.into_inner();
    service.update_note(user.0, note_id, payload.into_inner()).await
}

#[delete("/{note_id}")]
async fn delete_note(
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    service: web::Data<NoteService>
) -> Result<HttpResponse, Error> {
    let note_id = path.into_inner();
    service.delete_note(user.0, note_id).await
}

pub fn configure_notes_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notes")
            .wrap(from_fn(auth_middleware))
            .service(get_notes)
            .service(get_note)
            .service(create_note)
            .service(update_note)
            .service(delete_note)
    );
}