use crate::{algo::token_bucket::TokenBucketLimiter, ratelimiter::RateLimiter};

pub enum RateLimiterAlgo {
    TokenBucket(TokenBucketLimiter),
}

impl RateLimiter for RateLimiterAlgo {
    fn is_allowed(&mut self) -> bool {
        match self {
            RateLimiterAlgo::TokenBucket(token_bucket) => token_bucket.is_allowed(),
        }
    }
}
