# tt-crypto-primitives

Blockchain crypto primitives optimized for RISC-V, targeting Tenstorrent hardware acceleration.

**Status:** Phase 1 scaffold — software implementations with benchmark harness. RISC-V Zkn acceleration in progress.

## Primitives

| Primitive | Use in Blockchain | RISC-V Acceleration | Status |
|---|---|---|---|
| Keccak-256 | Ethereum's hash (addresses, txns, Merkle) | Zbb (bit manipulation) | Implemented |
| SHA-256 | Bitcoin, Merkle trees, integrity | Zknh (hardware SHA) | Implemented |
| ECDSA secp256k1 | Ethereum transaction signing | Zbb + software | Planned |
| NTT/iNTT | ZK proof generation | Vector + Tensix offload | Implemented |
| Polynomial multiply | FHE bootstrapping | Tensix matrix engine | Implemented |

## NTT Polynomial Multiplication

The `ntt` module implements a 256-point Number Theoretic Transform over
`q = 3329` using Kyber's primitive root `17`.

```rust
use tt_crypto_primitives::{multiply_polynomials, Polynomial};

let lhs: Polynomial = [1; 256];
let rhs: Polynomial = [2; 256];
let product = multiply_polynomials(&lhs, &rhs);
```

The helper multiplies polynomials in the cyclic ring `Zq[x] / (x^256 - 1)`.
It includes:

- `forward_ntt` and `inverse_ntt` for coefficient/evaluation conversion
- `multiply_polynomials` for NTT-based multiplication
- `multiply_polynomials_naive` as an O(n^2) reference implementation
- tests for Kyber reference root properties and zeta generation, inverse round
  trips, sparse basis multiplication, and NTT-vs-naive multiplication

## Feature Flags

```toml
[dependencies]
tt-crypto-primitives = { version = "0.1", features = ["riscv-zkn"] }
```

| Feature | What |
|---|---|
| `std` (default) | Standard library support |
| `riscv-zkn` | Use RISC-V Zkn scalar crypto instructions |
| `riscv-zvk` | Use RISC-V Zvk* vector crypto instructions |
| `tenstorrent` | Offload to TT-Metalium kernels (Phase 2+) |

## Benchmarks

```bash
cargo bench
```

Benchmarks use [Criterion](https://github.com/bheisler/criterion.rs) and test
hash inputs across 32B to 4KB, plus NTT polynomial multiplication against the
naive O(n^2) baseline.

## Validation

```bash
cargo test
cargo test --no-default-features
cargo bench
```

## Cross-compilation for RISC-V

```bash
# Install RISC-V target
rustup target add riscv64gc-unknown-linux-gnu

# Build for RISC-V
cargo build --target riscv64gc-unknown-linux-gnu

# Run on Whisper ISS
whisper --isa rv64gcv_zkn target/riscv64gc-unknown-linux-gnu/debug/tt-crypto-primitives
```

## Related

- [kcolbchain/riscv-ocelot](https://github.com/kcolbchain/riscv-ocelot) — Ocelot fork with crypto benchmarks
- [kcolbchain/fhe](https://github.com/kcolbchain/fhe) — FHE library (RISC-V acceleration roadmap)
- [tenstorrent/whisper](https://github.com/tenstorrent/whisper) — RISC-V ISS
- [tenstorrent/tt-metal](https://github.com/tenstorrent/tt-metal) — TT-Metalium kernel programming

## License

Apache-2.0
