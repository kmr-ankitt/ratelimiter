pub struct TokenBucket {
    capacity: usize,
    tokens: usize,
    refill_rate: usize
}

pub fn run() {
    println!("Token Bucket Rate Limiter");
}
