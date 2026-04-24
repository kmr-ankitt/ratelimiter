use actix_web::{HttpResponse, HttpServer, Responder, get, web};
use std::sync::Mutex;

use crate::{
    algo::{
        RateLimiterAlgo, fixed_window::FixedWindowLimiter, leaky_bucket::LeakyBucketLimiter,
        token_bucket::TokenBucketLimiter,
    },
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

#[get("/limited/fw")]
async fn limited_fw(data: web::Data<Mutex<RateLimiterAlgo>>) -> impl Responder {
    let mut ratelimiter = data.lock().unwrap();
    if ratelimiter.is_allowed() {
        HttpResponse::Ok().body("Allowed! Welcome :)")
    } else {
        HttpResponse::TooManyRequests().body("Too many requests! Please try again later.")
    }
}

#[get("/limited/swl")]
async fn limited_swl(data: web::Data<Mutex<RateLimiterAlgo>>) -> impl Responder {
    let mut ratelimiter = data.lock().unwrap();
    if ratelimiter.is_allowed() {
        HttpResponse::Ok().body("Allowed! Welcome :)")
    } else {
        HttpResponse::TooManyRequests().body("Too many requests! Please try again later.")
    }
}

#[get("/limited/swc")]
async fn limited_swc(data: web::Data<Mutex<RateLimiterAlgo>>) -> impl Responder {
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

    let leaky_bucket_rate_limiter = web::Data::new(Mutex::new(RateLimiterAlgo::LeakyBucket(
        LeakyBucketLimiter::new(5, 1.0),
    )));

    let fixed_window_rate_limiter = web::Data::new(Mutex::new(RateLimiterAlgo::FixedWindow(
        FixedWindowLimiter::new(5, std::time::Duration::from_secs(5)),
    )));

    let sliding_window_log_rate_limiter =
        web::Data::new(Mutex::new(RateLimiterAlgo::SlidingWindowLog(
            crate::algo::sliding_window_log::SlidingWindowLogLimiter::new(5, 5),
        )));

    let sliding_window_counter_rate_limiter =
        web::Data::new(Mutex::new(RateLimiterAlgo::SlidingWindowCounter(
            crate::algo::sliding_window_counter::SlidingWindowCounterLimiter::new(
                5,
                std::time::Duration::from_secs(5),
            ),
        )));

    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(token_bucket_rate_limiter.clone())
            .app_data(leaky_bucket_rate_limiter.clone())
            .app_data(fixed_window_rate_limiter.clone())
            .app_data(sliding_window_log_rate_limiter.clone())
            .app_data(sliding_window_counter_rate_limiter.clone())
            .service(index)
            .service(limited_tb)
            .service(limited_lb)
            .service(limited_fw)
            .service(limited_swl)
            .service(limited_swc)
            .service(unlimited)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
