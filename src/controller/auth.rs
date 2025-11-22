use crate::db::user;
use crate::{AppState, db};
use actix_web::{HttpResponse, Responder, post, web};
//use actix_web::error::ParseError::Header;
//use actix_web::web::Header;
//use actix_web::web::Header;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[post("auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db = state.db.lock().unwrap();

    if db::user::has_with_email(&db, &data.email).await {
        println!("User already exists!");
        return HttpResponse::UnprocessableEntity()
            .json(json!({"status":"error","message":"User already exists!"}));
    }

    let result = user::create(&db, &data).await;

    return if result {
        HttpResponse::Created().json(json!({"status":"success","message":"User created!"}))
    } else {
        HttpResponse::BadRequest()
            .json(json!({"status":"error","message":"Something went wrong during insert database"}))
    };
}

#[post("auth/sign-in")]
pub async fn sign_in(state: web::Data<AppState>, data: web::Json<SignInRequest>) -> impl Responder {
    let db = state.db.lock().unwrap();

    let Some(user) = db::user::get_by_email(&db, &data.email).await else {
        return HttpResponse::BadRequest()
            .json(json!({"status":"error","message":"Invalid email"}));
    };
    println!("User data is {:?}", user);

    if !bcrypt::verify(&data.password, &user.password).unwrap() {
        return HttpResponse::Unauthorized()
            .json(json!({"status":"error","message":"Un-authorize"}));
    };

    let claims = Claims {
        sub: user.id,
        exp: 10000000000,
        role: "user".to_string(),
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .unwrap();

    HttpResponse::Ok().json(json!({ "status":"success","token": token }))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
    pub role: String,
}
