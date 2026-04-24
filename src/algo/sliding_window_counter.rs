/*!
# Sliding Window Counter Rate Limiter

- Uses a sliding window approach to approximate request count.
- Maintains two counters:
  - `current_window_count`
  - `prev_window_count`
- Calculates a weighted count based on elapsed time within the window.
- Previous window contribution decays linearly over time.

## Formula
`weighted_count = current_window_count + prev_window_count * (1 - (elapsed_time / window_size))`

## Behavior
- If `weighted_count < limit` → request is allowed
- Otherwise → request is denied
*/

use std::time::{Duration, Instant};

use crate::ratelimiter::RateLimiter;

pub struct SlidingWindowCounterLimiter {
    limit: usize,
    window: Duration,
    window_start: Instant,

    current_window_count: usize,
    prev_window_count: usize,
}

impl SlidingWindowCounterLimiter {
    pub fn new(limit: usize, window: Duration) -> Self {
        Self {
            limit,
            window,
            window_start: Instant::now(),
            current_window_count: 0,
            prev_window_count: 0,
        }
    }

    fn update_window(&mut self, now: Instant) {
        if now.duration_since(self.window_start) >= self.window {
            self.prev_window_count = self.current_window_count;
            self.current_window_count = 0;
            self.window_start = now;
        }
    }

    fn calculate_weighted_count(&self, now: Instant) -> f64 {
        let elapsed = now.duration_since(self.window_start);

        let weight = elapsed.as_secs_f64() / self.window.as_secs_f64();

        self.current_window_count as f64 + self.prev_window_count as f64 * (1.0 - weight)
    }
}

impl RateLimiter for SlidingWindowCounterLimiter {
    fn is_allowed(&mut self) -> bool {
        let now = Instant::now();
        self.update_window(now);
        let weighted_count = self.calculate_weighted_count(now);
        if weighted_count < self.limit as f64 {
            self.current_window_count += 1;
            true
        } else {
            false
        }
    }
}
