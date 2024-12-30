# Measure bump heap allocation
**Purpose:** An experience for optimizing CU, increase the available heap memory for CPI calls between programs

For rust native
```bash
cd rust-native && cargo run release
```

For solana program
```bash
cargo test --manifest-path tests/Cargo.toml -- --nocapture
```

**Result amount of CU optimized**

*Heap Allocation*
```rust
Allocate 1024 bytes consumed 1500 compute units
Allocate 10240 bytes consumed 1800 compute units
Allocate 102400 bytes consumed 2200 compute units
```

*Optimizing Bump Heap Allocation*
```rust
Allocate 1024 bytes consumed 1200 compute units
Allocate 10240 bytes consumed 1600 compute units
Allocate 102400 bytes consumed 2000 compute units
```
