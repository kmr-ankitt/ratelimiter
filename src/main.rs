use actix_web::{HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, weaks!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8000");

    HttpServer::new(|| actix_web::App::new().service(index))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
