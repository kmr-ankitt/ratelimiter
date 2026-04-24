use crate::{
    algo::{
        fixed_window::FixedWindowLimiter, leaky_bucket::LeakyBucketLimiter,
        token_bucket::TokenBucketLimiter,
    },
    ratelimiter::RateLimiter,
};

pub enum RateLimiterAlgo {
    TokenBucket(TokenBucketLimiter),
    LeakyBucket(LeakyBucketLimiter),
    FixedWindow(FixedWindowLimiter),
}

impl RateLimiter for RateLimiterAlgo {
    fn is_allowed(&mut self) -> bool {
        match self {
            RateLimiterAlgo::TokenBucket(token_bucket) => token_bucket.is_allowed(),
            RateLimiterAlgo::LeakyBucket(leaky_bucket) => leaky_bucket.is_allowed(),
            RateLimiterAlgo::FixedWindow(fixed_window) => fixed_window.is_allowed(),
        }
    }
}
