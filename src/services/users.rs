use actix_session::Session;
use sqlx::PgPool;
use actix_web::{HttpResponse, Result, Error};
use email_address::EmailAddress;
use serde_json::json;
use crate::utils::verify_password;
use crate::models::{LoginRequest, NewUser, RegistrationRequest, User, UserResponse};
use crate::repositories::UserRepository;

pub struct UserService {
    pub repo: UserRepository
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: UserRepository::new(pool)
        }
    }

    fn validate_email(&self, email: &str) -> bool {
        EmailAddress::is_valid(email)
    }

    fn validate_password(&self, password: &str) -> bool {
        password.len() >= 8
            && password.chars().any(|c| c.is_uppercase())
            && password.chars().any(|c| c.is_lowercase())
            && password.chars().any(|c| c.is_ascii_digit())
            && password.chars().any(|c| "!@#$%^&*".contains(c))
    }

    async fn authenticate_user(&self, credentials: &LoginRequest) -> Result<Option<User>, String> {
        let user = self.repo.find_by_email(&credentials.email).await.map_err(|e| e.to_string())?;

        if let Some(user) = user {
            match verify_password(&credentials.password, &user.password_hash) {
                Ok(true) => Ok(Some(user)),
                Ok(false) => Ok(None),
                Err(e) => Err(e.to_string())
            }
        } else {
            Ok(None)
        }
    }

    pub async fn register_user(&self, user_info: RegistrationRequest) -> Result<HttpResponse, Error> {
        let existing_user = self.repo.find_by_email(&user_info.email).await;
        
        if existing_user.is_err() {
            return Ok(HttpResponse::InternalServerError().json(json!({"message": "Internal server error"})));
        }

        if existing_user.unwrap().is_some() {
            return Ok(HttpResponse::Conflict().json(json!({"message": "User already exists"})));
        }

        if !self.validate_email(&user_info.email) {
            return Ok(HttpResponse::BadRequest().json(json!({"message": "Invalid email"})));
        }

        if !self.validate_password(&user_info.password) {
            return Ok(HttpResponse::BadRequest().json(json!({"message": "Invalid password"})));
        }

        match self.repo.create_user(NewUser::try_from(user_info).unwrap()).await {
            Ok(user) => {
                let user_response = UserResponse::from(user);
                Ok(HttpResponse::Created().json(user_response))
            },
            Err(_) => Ok(HttpResponse::InternalServerError().json(json!({"message": "Failed to create user"})))
        }
    }

    pub async fn login_user(&self, credentials: LoginRequest, session: Session) -> Result<HttpResponse, Error> {
        match self.authenticate_user(&credentials).await {
            Ok(Some(user)) => {
                // Store user info in session
                session.insert("user_id", user.id)?;
                session.insert("logged_in", true)?;

                Ok(HttpResponse::Ok().json(UserResponse::from(user)))
            },
            Ok(None) => {
                Ok(HttpResponse::Unauthorized().json(json!({"message": "Invalid credentials"})))
            },
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(json!({"message": e})))
            }
        }
    }

    pub async fn logout_user(&self, session: Session) -> Result<HttpResponse, Error> {
        session.purge();
        Ok(HttpResponse::Ok().json(json!({"message": "Successfully logged out"})))
    }

    pub async fn get_current_user(&self, session: Session) -> Result<HttpResponse, Error> {
        // Check if user is logged in
        let is_logged_in = session
            .get::<bool>("logged_in")?
            .unwrap_or(false);

        if !is_logged_in {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "message": "Not logged in"
            })));
        }

        // Get user ID from session
        let user_id = session.get::<uuid::Uuid>("user_id")?;

        let Some(user_id) = user_id else {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "message": "Not logged in"
            })));
        };

        // Find user in database
        let user = self.repo
            .find_by_id(user_id)
            .await;

        match user {
            Ok(Some(user)) => Ok(HttpResponse::Ok().json(UserResponse::from(user))),
            Ok(None) => {
                // User doesn't exist anymore, clean up session
                session.purge();
                Ok(HttpResponse::Unauthorized().json(json!({
                    "message": "Session invalid"
                })))
            },
            Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
                "message": "Internal server error. Check logs."
            })))
        }
    }
}