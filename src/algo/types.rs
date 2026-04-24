use crate::{
    algo::{
        fixed_window::FixedWindowLimiter, leaky_bucket::LeakyBucketLimiter,
        sliding_window_counter::SlidingWindowCounterLimiter,
        sliding_window_log::SlidingWindowLogLimiter, token_bucket::TokenBucketLimiter,
    },
    ratelimiter::RateLimiter,
};

pub enum RateLimiterAlgo {
    TokenBucket(TokenBucketLimiter),
    LeakyBucket(LeakyBucketLimiter),
    FixedWindow(FixedWindowLimiter),
    SlidingWindowLog(SlidingWindowLogLimiter),
    SlidingWindowCounter(SlidingWindowCounterLimiter),
}

impl RateLimiter for RateLimiterAlgo {
    fn is_allowed(&mut self) -> bool {
        match self {
            RateLimiterAlgo::TokenBucket(token_bucket) => token_bucket.is_allowed(),
            RateLimiterAlgo::LeakyBucket(leaky_bucket) => leaky_bucket.is_allowed(),
            RateLimiterAlgo::FixedWindow(fixed_window) => fixed_window.is_allowed(),
            RateLimiterAlgo::SlidingWindowLog(sliding_window_log) => {
                sliding_window_log.is_allowed()
            }
            RateLimiterAlgo::SlidingWindowCounter(sliding_window_counter) => {
                sliding_window_counter.is_allowed()
            }
        }
    }
}
