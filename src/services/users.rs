use actix_session::Session;
use sqlx::PgPool;
use actix_web::{HttpResponse, Result};
use email_address::EmailAddress;
use serde_json::json;
use uuid::Uuid;
use crate::utils::verify_password;
use crate::models::{LoginRequest, NewUser, RegistrationRequest, UserResponse, UserSession};
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

    fn create_session_key(&self) -> Uuid {
        Uuid::new_v4()
    }

    async fn authenticate_user(&self, credentials: &LoginRequest) -> Result<bool, String> {
        let user = self.repo.find_by_email(credentials.email.as_str()).await.map_err(
            |e| e.to_string()
        )?;
        if let Some(user) = user {
            match verify_password(&*credentials.password, &*user.password_hash) {
                Ok(true) => Ok(true),
                Ok(false) => Ok(false),
                Err(e) => Err(e.to_string())
            }
        } else {
            Ok(false)
        }
    }

    pub async fn register_user(&self, user_info: RegistrationRequest) -> Result<HttpResponse, String> {
        let user = self.repo.find_by_email(&*user_info.email).await.map_err(
            |e| e.to_string()
        )?;

        if let Some(user) = user {
            Ok(HttpResponse::Conflict().json(
                json!(
                    {
                        "message": "User already exists"
                    }
                )
            ))
        } else {
            let email_valid = self.validate_email(&*user_info.email);
            let password_valid = self.validate_password(&*user_info.password);

            if !email_valid {
                return Ok(HttpResponse::BadRequest().json(
                    json!(
                        {
                            "message": "Invalid email"
                        }
                    )
                ));
            }

            if !password_valid {
                return Ok(HttpResponse::BadRequest().json(
                    json!(
                        {
                            "message": "Invalid password"
                        }
                    )
                ));
            }

            let user = self.repo.create_user(NewUser::try_from(user_info)?).await.map_err(
                |e| e.to_string()
            )?;
            let user_response = UserResponse::from(user);
            Ok(HttpResponse::Created().json(user_response))
        }
    }

    pub async fn login_user(
        &self,
        credentials: LoginRequest,
        session: Session
    ) -> Result<HttpResponse, String> {
        match self.authenticate_user(&credentials).await {
            Ok(false) => Ok(HttpResponse::Unauthorized().json(
                json!(
                    {
                        "message": "Invalid credentials"
                    }
                )
            )),
            Ok(true) => {
                let user = self.repo.find_by_email(&*credentials.email).await.map_err(
                    |e| e.to_string()
                )?.unwrap();
                let session_id = self.create_session_key();
                session.insert(session_id, UserSession::from(user.clone())).expect("Failed to create session");
                Ok(HttpResponse::Ok().json(UserResponse::from(user.clone())))
            },
            Err(e) => Ok(HttpResponse::InternalServerError().json(
                json!(
                    {
                        "message": e
                    }
                )
            ))
        }
    }

    pub async fn logout_user(&self, session: Session, session_id: Uuid) -> Result<HttpResponse, String> {
        // validate session id
        let user_session = session.get::<UserSession>(session_id.to_string().as_str());
        match user_session {
            Ok(Some(user_session)) => {
                session.remove(session_id.to_string().as_str());
                Ok(HttpResponse::Ok().json(json!({ "message": "Successfully logged out" })))
            },
            Ok(None) => {
                Ok(HttpResponse::Unauthorized().json(json!({ "message": "Invalid session" })))
            },
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(json!({ "message": e.to_string() })))
            }
        }
    }
}