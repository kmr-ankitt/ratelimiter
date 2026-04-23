pub trait RateLimiter {
    fn is_allowed(&mut self) -> bool;
}
