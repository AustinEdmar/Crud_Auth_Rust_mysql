use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::body::BoxBody;
use actix_web::error::{Error, ErrorUnauthorized};
use actix_web::http::header::HeaderValue;
use actix_web::middleware::Next;
use actix_web::web;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use actix_web::HttpMessage;
use crate::controllers::auth::Claims;
use crate::AppState;

pub async fn verify_jwt(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse, Error> {

    let auth_header: &HeaderValue = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| {
            ErrorUnauthorized(json!({
                "status": "error",
                "message": "Unauthorized"
            }))
        })?;

    let auth_str = auth_header.to_str().map_err(|_| {
        ErrorUnauthorized(json!({
            "status": "error",
            "message": "Authorization header is invalid"
        }))
    })?;

    if !auth_str.starts_with("Bearer ") {
        return Err(ErrorUnauthorized(json!({
            "status": "error",
            "message": "Invalid Bearer token"
        })));
    }

    let token = auth_str.strip_prefix("Bearer ").unwrap();

    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let key = DecodingKey::from_secret(state.json_web_token.as_bytes());

    match decode::<Claims>(token, &key, &Validation::default()) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims.sub);
            Ok(next.call(req).await?)
        }
        Err(_) => Err(ErrorUnauthorized(json!({
            "status": "error",
            "message": "Invalid or expired token"
        }))),
    }
}
