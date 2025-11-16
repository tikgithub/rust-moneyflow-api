use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Application Starting");
    HttpServer::new(|| {
        App::new()
        //.service(hello)
        //.service(echo)
        //.route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
