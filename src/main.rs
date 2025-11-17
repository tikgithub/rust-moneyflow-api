use std::sync::Mutex;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

mod controller;
mod db;

struct AppState {
    db: Mutex<sqlx::PgPool>,

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Application Starting");
    dotenvy::dotenv().ok();

    let app_state = web::Data::new(AppState{
        db: Mutex::
        new(
            sqlx::PgPool::connect(&std::env::var("DATABASE_URL")
                .unwrap())
                .await.unwrap()
        ),
    });

    HttpServer::new( move || {
        App::new()
            .app_data(app_state.clone())
            .service(controller::auth::sign_in)
            .service(controller::auth::sign_up)
            .service(controller::me::get_profile)
            .service(controller::me::update_profile)
        //.route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
