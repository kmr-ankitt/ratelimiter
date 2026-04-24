/*! 
# Fixed Window Rate Limiter 

- Fixed window is a time based window where requests are counted in fixed time intervals
- It maintains a counter for each window and increments it when a request is made
- When the window expires, the counter is reset to 0 and the next window starts
- It is a simple and efficient algorithm, but it has a drawback that it can lead to burst traffic
*/

use std::time::{Duration, Instant};

use crate::ratelimiter::RateLimiter;

pub struct FixedWindowLimiter {
    capacity: usize,
    window_size: Duration,
    current_count: usize,
    window_start: Instant,
}

impl FixedWindowLimiter {
    pub fn new(capacity: usize, window_size: Duration) -> Self {
        Self {
            capacity,
            window_size,
            current_count: 0,
            window_start: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.current_count = 0;
        self.window_start = Instant::now();
    }
}

impl RateLimiter for FixedWindowLimiter {
    fn is_allowed(&mut self) -> bool {
        let now = Instant::now();

        if now.duration_since(self.window_start) >= self.window_size {
            self.reset();
        }

        if self.current_count < self.capacity {
            self.current_count += 1;
            true
        } else {
            false
        }
    }
}
