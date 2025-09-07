use actix_session::Session;
use actix_web::{post, web, HttpResponse, Error, get};
use actix_web::middleware::from_fn;
use crate::middleware::auth_middleware;
use crate::models::{LoginRequest, RegistrationRequest};
use crate::services::UserService;


#[post("/login")]
pub async fn login(
    session: Session,
    service: web::Data<UserService>,
    payload: web::Json<LoginRequest>
) -> Result<HttpResponse, Error> {
    service.login_user(payload.into_inner(), session).await
}

#[post("/logout")]
pub async fn logout(
    session: Session,
    service: web::Data<UserService>
) -> Result<HttpResponse, Error> {
    service.logout_user(session).await
}

#[post("/register")]
pub async fn register(
    service: web::Data<UserService>,
    payload: web::Json<RegistrationRequest>
) -> Result<HttpResponse, Error> {
    service.register_user(payload.into_inner()).await
}

#[get("/me")]
pub async fn me(
    session: Session,
    service: web::Data<UserService>
) -> Result<HttpResponse, Error> {
    service.get_current_user(session).await
}

pub fn configure_auth_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            // Public routes - no auth needed
            .service(login)
            .service(register)
            // Protected sub-scope
            .service(
                web::scope("")
                    .wrap(from_fn(auth_middleware))
                    .service(logout)
                    .service(me)
            )
    );
}