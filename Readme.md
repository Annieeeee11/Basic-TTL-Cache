# 🧪 TTL Cache in Rust — Learning Project
I built a small basic in-memory cache that stores key–value pairs with a time-to-live (TTL) and automatically removes expired data.

## What this cache does

- Stores key → value pairs in memory
- Each value has a TTL (time to live)
- Expired values are:
                    removed when accessed
                    or removed by calling cleanup()
- Tracks:
        how many times a key was found (hits)
        how many times it was missing or expired (misses)

## Project structure
```
src/
├── lib.rs 
├── cache.rs  
├── entry.rs
└── time.rs  
└── benches/
    └── cache_bench.rs  
```

## How to run tests
```
cargo test
```