# Contributing to tt-crypto-primitives

We're building the first blockchain crypto library optimized for RISC-V hardware. Contributions welcome from anyone working at the intersection of blockchain, cryptography, and open hardware.

## What We Need

| Area | Skills | Issues |
|---|---|---|
| **Crypto primitives** | Rust, cryptography | ECDSA, NTT, polynomial arithmetic |
| **RISC-V acceleration** | RISC-V ISA, assembly, intrinsics | Zkn/Zvk* instruction integration |
| **Benchmarking** | Rust, performance analysis | Criterion benchmarks, cross-platform comparison |
| **FHE integration** | FHE schemes (TFHE, BGV), Rust FFI | TT-Metalium kernel mapping |
| **Testing** | Rust, test vectors | NIST vectors, cross-implementation validation |

## Getting Started

```bash
# Clone and test
git clone https://github.com/kcolbchain/tt-crypto-primitives.git
cd tt-crypto-primitives
cargo test

# Run benchmarks
cargo bench

# Cross-compile for RISC-V (optional)
rustup target add riscv64gc-unknown-linux-gnu
cargo build --target riscv64gc-unknown-linux-gnu
```

## Pull Request Process

1. Fork the repo and create a branch from `main`
2. Write tests for any new primitive (reference vectors required)
3. Run `cargo test` and `cargo clippy`
4. Open a PR with a clear description of what and why

## Code Style

- `#![no_std]` compatible where possible (feature-gate `std` usage)
- Use `wrapping_add`, `rotate_left` etc. — no panicking arithmetic in crypto
- Every primitive needs at least 2 test vectors from a reference source
- Benchmark every primitive via Criterion

## Feature Flags

When adding RISC-V-specific code, gate it behind the appropriate feature:
- `riscv-zkn` — scalar crypto (AES, SHA via Zkn* instructions)
- `riscv-zvk` — vector crypto (Zvk* instructions)
- `tenstorrent` — TT-Metalium offload (Phase 2+)

Software fallbacks must always exist for non-RISC-V targets.

## License

All contributions are under Apache-2.0.
