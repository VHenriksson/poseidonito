# Project Layout

```
Cargo.toml          - workspace manifest
src/                - core library implementation
   configurations/  - Poseidon configuration modules
   permutation.rs   - generic permutation logic
   sponge.rs        - generic sponge construction
   lib.rs           - public API
profiling/          - small binary crate to benchmark hashing
```

The main library exposes a single preset configuration `x5_254_3` in
`src/configurations/config_x5_254_3/`.
