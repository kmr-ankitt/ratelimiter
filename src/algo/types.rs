use crate::{
    algo::{leaking_bucket::LeakingBucketLimiter, token_bucket::TokenBucketLimiter},
    ratelimiter::RateLimiter,
};

pub enum RateLimiterAlgo {
    TokenBucket(TokenBucketLimiter),
    LeakingBucket(LeakingBucketLimiter),
}

impl RateLimiter for RateLimiterAlgo {
    fn is_allowed(&mut self) -> bool {
        match self {
            RateLimiterAlgo::TokenBucket(token_bucket) => token_bucket.is_allowed(),
            RateLimiterAlgo::LeakingBucket(leaking_bucket) => leaking_bucket.is_allowed(),
        }
    }
}
