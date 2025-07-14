# Building & Testing

First install the Rust toolchain from [rustup.rs](https://rustup.rs/) if you
haven't already. Once Rust is installed you can run:

```bash
cargo test
```

This compiles the library and runs all unit tests. If you want to profile the
hash implementation you can run the profiling crate:

```bash
cargo run -p profiling
```
