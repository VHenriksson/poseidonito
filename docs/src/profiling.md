# Profiling Example

The workspace contains a small `profiling` crate which hashes the numbers
`1..99_999` using the provided `x5_254_3` configuration. You can run it with:

```bash
cargo run -p profiling
```

This is a simple way to measure performance or use the hash in a standalone
context.
