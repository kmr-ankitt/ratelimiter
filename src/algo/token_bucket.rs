/*!
# Token Bucket Algorithm
- Bucket has fixed capacity (`bucket_size`)
- Bucket will follow lazy refill strategy (refill only when a request is made)
- Tokens are refilled at a constant rate (`refill_rate/second`)
- Each request consumes 1 token
- If no tokens are available → request is denied
- Supports burst traffic up to bucket capacity

Reference: [wiki](https://en.wikipedia.org/wiki/Token_bucket)
*/

use std::time::Instant;

use crate::ratelimiter::RateLimiter;

pub struct TokenBucketLimiter {
    bucket_size: usize,
    refill_rate: f64,
    tokens: usize,
    last_refill_time: Instant,
}

impl TokenBucketLimiter {
    pub fn new(bucket_size: usize, refill_rate: f64) -> Self {
        Self {
            bucket_size,
            refill_rate,
            tokens: bucket_size,
            last_refill_time: Instant::now(),
        }
    }

    fn consume(&mut self) -> bool {
        self.refill();

        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill_time).as_secs_f64();

        let new_tokens = elapsed * self.refill_rate;
        self.tokens = (new_tokens + self.tokens as f64).min(self.bucket_size as f64) as usize;

        self.last_refill_time = now;
    }
}

impl RateLimiter for TokenBucketLimiter {
    fn is_allowed(&mut self) -> bool {
        self.consume()
    }
}
