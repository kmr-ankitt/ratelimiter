use actix_web::{HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, weaks!")
}

#[get("/limited")]
async fn limited() -> impl Responder {
    HttpResponse::Ok().body("Limited! Uh no :(")
}

#[get("/unlimited")]
async fn unlimited() -> impl Responder {
    HttpResponse::Ok().body("Unlimited! Let's go :)")
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    println!("Server running at http://localhost:8000");

    HttpServer::new(|| {
        actix_web::App::new()
            .service(index)
            .service(limited)
            .service(unlimited)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
