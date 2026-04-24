# Ratelimiter

> A rate limiter service built in Rust using multiple algorithms.
> Designed to be fast, modular, scalable and easy to use.

## Algorithms

- [x] [Token Bucket Algorithm]("https://en.wikipedia.org/wiki/Token_bucket")
- [x] [Leaky Bucket Algorithm]("https://en.wikipedia.org/wiki/Leaky_bucket_algorithm")
- [x] [Fixed Window Algorithm]("https://en.wikipedia.org/wiki/Fixed_window_algorithm")
- [ ] [Sliding Window Algorithm]("https://en.wikipedia.org/wiki/Sliding_window_algorithm")

## Usage

```bash
cargo run
```

Server runs on `http://localhost:8000`

## Endpoints

- `GET /` - A simple hello world route.
- `GET /unlimited` - An endpoint without any rate limiting.
- `GET /limited/tb` - An endpoint protected by the Token Bucket algorithm.
- `GET /limited/lb` - An endpoint protected by the Leaky Bucket algorithm.
- `GET /limited/fw` - An endpoint protected by the Fixed Window algorithm.
