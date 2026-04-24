use actix_web::{HttpResponse, HttpServer, Responder, get, web};
use std::sync::Mutex;

use crate::{
    algo::{RateLimiterAlgo, leaking_bucket, token_bucket::TokenBucketLimiter},
    ratelimiter::RateLimiter,
};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, ppl!")
}

#[get("/limited/tb")]
async fn limited_tb(data: web::Data<Mutex<RateLimiterAlgo>>) -> impl Responder {
    let mut rate_limiter = data.lock().unwrap();
    if rate_limiter.is_allowed() {
        HttpResponse::Ok().body("Allowed! Welcome :)")
    } else {
        HttpResponse::TooManyRequests().body("Too many requests! Please try again later.")
    }
}

#[get("/limited/lb")]
async fn limited_lb(data: web::Data<Mutex<RateLimiterAlgo>>) -> impl Responder {
    let mut ratelimiter = data.lock().unwrap();
    if ratelimiter.is_allowed() {
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

    let token_bucket_rate_limiter = web::Data::new(Mutex::new(RateLimiterAlgo::TokenBucket(
        TokenBucketLimiter::new(5, 1.0),
    )));

    let leaking_bucket_rate_limiter = web::Data::new(Mutex::new(RateLimiterAlgo::LeakingBucket(
        leaking_bucket::LeakingBucketLimiter::new(5, 1.0),
    )));

    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(token_bucket_rate_limiter.clone())
            .app_data(leaking_bucket_rate_limiter.clone())
            .service(index)
            .service(limited_tb)
            .service(limited_lb)
            .service(unlimited)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
