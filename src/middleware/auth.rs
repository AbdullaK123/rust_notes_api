use actix_session::SessionExt;
use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{HttpResponse};
use actix_web::middleware::Next;
use actix_web::Error;
use serde_json::json;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    // get the users session from the service request
    let session = req.get_session();

    let is_logged_in = session
        .get::<bool>("logged_in")
        .unwrap_or(Some(false))
        .unwrap_or(false);
    
    // if its valid call the next service in the chain otherwise return a 401
    if is_logged_in {
        let res = next.call(req).await?;
        Ok(res.map_into_boxed_body())
    } else {
        let response = HttpResponse::Unauthorized().json(
            json!(
                {
                    "message": "Not Authenticated"
                }
            )
        );
        Ok(req.into_response(response))
    }
}