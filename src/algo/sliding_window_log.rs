/*!
# Sliding Window Log Rate Limiter
- This implementation uses a log of timestamps in Deque to track requests within a sliding window.
- It prunes old logs that fall outside the window and checks if the number of logs is within the allowed limit.
- This approach is more accurate than the fixed window counter but may consume more memory if there are many requests.
*/

use std::{
    collections::VecDeque,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::ratelimiter::RateLimiter;

pub struct SlidingWindowLogLimiter {
    logs: VecDeque<u64>,
    window_size: u64,
    limit: usize,
}

impl SlidingWindowLogLimiter {
    pub fn new(window_size: u64, limit: usize) -> Self {
        Self {
            logs: VecDeque::new(),
            window_size,
            limit,
        }
    }

    fn prune_logs(&mut self, current_time: u64) {
        while let Some(&timestamp) = self.logs.front() {
            if current_time - timestamp > self.window_size {
                self.logs.pop_front();
            } else {
                break;
            }
        }
    }

    fn add_log(&mut self, timestamp: u64) {
        self.logs.push_back(timestamp);
    }

    fn count_logs(&self) -> usize {
        self.logs.len()
    }

    fn current_time() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl RateLimiter for SlidingWindowLogLimiter {
    fn is_allowed(&mut self) -> bool {
        let current_time = Self::current_time();
        self.prune_logs(current_time);

        if self.count_logs() < self.limit {
            self.add_log(current_time);
            true
        } else {
            false
        }
    }
}
