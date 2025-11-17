use std::sync::{Mutex, MutexGuard};
use actix_web::{Responder, post, web};
use serde::{Deserialize, Serialize};
use crate::{db, AppState};

#[post("auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db = state.db.lock().unwrap();;

    if db::user::has_with_email(&db, &data.email).await{
        return "User already exists".to_string();
    }

    format!("SignUp {:?}", data)
}

#[post("auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    return "Sign-in";
}

#[derive(Deserialize,Serialize,Debug)]
struct SignUpRequest {
    email: String,
    password: String,
    firstname: String,
    lastname: String,
}
