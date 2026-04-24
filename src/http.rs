use actix_web::{HttpResponse, HttpServer, Responder, get, web};
use std::sync::Mutex;

use crate::{
    algo::{
        RateLimiterAlgo, fixed_window::FixedWindowLimiter, leaky_bucket::LeakyBucketLimiter,
        sliding_window_counter::SlidingWindowCounterLimiter,
        sliding_window_log::SlidingWindowLogLimiter, token_bucket::TokenBucketLimiter,
    },
    ratelimiter::RateLimiter,
};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, ppl!")
}

#[get("/unlimited")]
async fn unlimited() -> impl Responder {
    HttpResponse::Ok().body("Unlimited! Let's go :)")
}

#[get("")]
async fn limited(data: web::Data<Mutex<RateLimiterAlgo>>) -> impl Responder {
    let mut rl = data.lock().unwrap();

    if rl.is_allowed() {
        HttpResponse::Ok().body("Allowed! Welcome :)")
    } else {
        HttpResponse::TooManyRequests().body("Too many requests! Please try again later.")
    }
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    println!("Server running at http://localhost:8000");

    let tb = web::Data::new(Mutex::new(RateLimiterAlgo::TokenBucket(
        TokenBucketLimiter::new(5, 1.0),
    )));

    let lb = web::Data::new(Mutex::new(RateLimiterAlgo::LeakyBucket(
        LeakyBucketLimiter::new(5, 1.0),
    )));

    let fw = web::Data::new(Mutex::new(RateLimiterAlgo::FixedWindow(
        FixedWindowLimiter::new(5, std::time::Duration::from_secs(5)),
    )));

    let swl = web::Data::new(Mutex::new(RateLimiterAlgo::SlidingWindowLog(
        SlidingWindowLogLimiter::new(5, 5),
    )));

    let swc = web::Data::new(Mutex::new(RateLimiterAlgo::SlidingWindowCounter(
        SlidingWindowCounterLimiter::new(5, std::time::Duration::from_secs(5)),
    )));

    HttpServer::new(move || {
        actix_web::App::new()
            .service(index)
            .service(unlimited)
            // Token Bucket
            .service(
                web::scope("/limited/tb")
                    .app_data(tb.clone())
                    .service(limited),
            )
            // Leaky Bucket
            .service(
                web::scope("/limited/lb")
                    .app_data(lb.clone())
                    .service(limited),
            )
            // Fixed Window
            .service(
                web::scope("/limited/fw")
                    .app_data(fw.clone())
                    .service(limited),
            )
            // Sliding Window Log
            .service(
                web::scope("/limited/swl")
                    .app_data(swl.clone())
                    .service(limited),
            )
            // Sliding Window Counter
            .service(
                web::scope("/limited/swc")
                    .app_data(swc.clone())
                    .service(limited),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
