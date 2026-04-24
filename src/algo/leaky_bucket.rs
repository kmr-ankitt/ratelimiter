/*!
# Leaky Bucket Algorithm
- Bucket has fixed capacity (`bucket_size`)
- Water leaks out at a constant rate (`outflow_rate/second`)
- Each request adds 1 unit of water to the bucket
- If bucket overflows → request is denied
- Supports burst traffic up to bucket capacity

Reference: [wiki](https://en.wikipedia.org/wiki/Leaky_bucket)
*/

use std::time::Instant;

use crate::ratelimiter::RateLimiter;

pub struct LeakyBucketLimiter {
    bucket_size: usize,
    outflow_rate: f64,
    water: f64,
    last_leak_time: Instant,
}

impl LeakyBucketLimiter {
    pub fn new(bucket_size: usize, outflow_rate: f64) -> Self {
        Self {
            bucket_size,
            outflow_rate,
            water: 0.0,
            last_leak_time: Instant::now(),
        }
    }

    pub fn leak(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_leak_time).as_secs_f64();

        let leaked_water = elapsed * self.outflow_rate;
        self.water = (self.water - leaked_water).max(0.0);

        self.last_leak_time = now;
    }

    pub fn add_water(&mut self) -> bool {
        self.leak();

        if self.water >= self.bucket_size as f64 {
            return false;
        }

        self.water += 1.0;
        true
    }
}

impl RateLimiter for LeakyBucketLimiter {
    fn is_allowed(&mut self) -> bool {
        self.add_water()
    }
}
