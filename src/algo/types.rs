use crate::algo::token_bucket::TokenBucket;

pub enum RateLimiterAlgo {
    Token(TokenBucket),
}
