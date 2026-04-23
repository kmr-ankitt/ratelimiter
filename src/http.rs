use actix_web::{HttpResponse, HttpServer, Responder, get};

use crate::{
    algo::{RateLimiterAlgo, token_bucket::TokenBucketLimiter},
    ratelimiter::RateLimiter,
};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, ppl!")
}

#[get("/limited")]
async fn limited() -> impl Responder {
    let mut rate_limiter = RateLimiterAlgo::TokenBucket(TokenBucketLimiter::new(5, 1.0));
    if rate_limiter.is_allowed() {
        HttpResponse::Ok().body("Allowed! Welcome :)")
    } else {
        HttpResponse::TooManyRequests().body("Too many requests! Please try again later.")
    }
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
