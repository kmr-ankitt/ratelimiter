# Ratelimiter

> A rate limiter service built in Rust using multiple algorithms.
> Designed to be fast, modular, scalable and easy to use.

## Algorithms

- [x] [Token Bucket Algorithm]("https://en.wikipedia.org/wiki/Token_bucket")
- [ ] [Leaky Bucket Algorithm]("https://en.wikipedia.org/wiki/Leaky_bucket_algorithm")
- [ ] [Fixed Window Algorithm]("https://en.wikipedia.org/wiki/Fixed_window_algorithm")
- [ ] [Sliding Window Algorithm]("https://en.wikipedia.org/wiki/Sliding_window_algorithm")

## Usage 

```bash
cargo run
```

Server runs on `http://localhost:8000`

## Endpoints

- `GET /` - A simple hello world route.
- `GET /limited` - A rate-limited endpoint. If you exceed the configured limit, it returns `429 Too Many Requests`.
- `GET /unlimited` - An endpoint without any rate limiting.
