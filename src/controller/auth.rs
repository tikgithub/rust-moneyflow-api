use actix_web::{Responder, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{db, AppState};
use crate::db::user;

#[post("auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db = state.db.lock().unwrap();;

    if db::user::has_with_email(&db, &data.email).await{
        println!("User already exists!");
        return HttpResponse::UnprocessableEntity().json(json!({"status":"error","message":"User already exists!"}));
    }

    let result = user::create(&db, &data).await;

    return if result {
        HttpResponse::Created().json(json!({"status":"success","message":"User created!"}))
    }else {
        HttpResponse::BadRequest().json(json!({"status":"error","message":"Something went wrong during insert database"}))
    }
}

#[post("auth/sign-in")]
pub async fn sign_in(state: web::Data<AppState>, data: web::Json<SignInRequest>) -> impl Responder {
    let db = state.db.lock().unwrap();

    let Some(user) = db::user::get_by_email(&db,&data.email).await else {
        return HttpResponse::BadRequest().json(json!({"status":"error","message":"Invalid email"}));
    };

    if(!bcrypt::verify(&data.password, &user.password).unwrap()){
        return HttpResponse::Unauthorized().json(json!({"status":"error","message":"Un-authorize"}));
    };

    ""
}

#[derive(Deserialize,Serialize,Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct SignInRequest{
    pub email: String,
    pub password: String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Claims{
    pub sub: u64,
    pub exp: u64,
    pub role: String
}
