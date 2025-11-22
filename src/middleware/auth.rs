use crate::AppState;
use crate::controller::auth::Claims;
use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::middleware::Next;
use actix_web::{Error, HttpMessage};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde_json::json;

pub async fn verify_jwt(req: ServiceRequest, next: Next<BoxBody>) -> Result<ServiceResponse, Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ErrorUnauthorized(json!({"status":"error", "message":"unauthorize"})))?;

    let auth_str = auth_header.to_str().map_err(|_| {
        ErrorUnauthorized(json!({"status":"error", "message":"authorize header is malformed"}))
    })?;

    if !auth_str.starts_with("Bearer ") {
        return Err(ErrorUnauthorized(
            json!({"status":"error", "message":"unauthorized token"}),
        ));
    }

    let token = auth_str.strip_prefix("Bearer ").unwrap();

    let state = req.app_data::<AppState>().unwrap();

    let key = DecodingKey::from_secret(&state.jwt_secret.as_bytes());

    match decode::<Claims>(token, &key, &Validation::default()) {
        Ok(token_data) => {
            //  let claim = tokenData.claims;
            req.extensions_mut().insert(token_data.claims.sub);

            next.call(req).await
        }
        Err(_) => {
            return Err(ErrorUnauthorized(
                json!({"status":"error", "message":"invalid token"}),
            ));
        }
    }
}
