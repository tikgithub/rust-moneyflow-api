use actix_web::{Responder, get, post};

#[get("/me")]
pub async fn get_profile() -> impl Responder {
    return "Profile";
}

#[post("/me")]
pub async fn update_profile() -> impl Responder {
    return "Update Profile";
}
