//! # tt-crypto-primitives
//!
//! Blockchain crypto primitives optimized for RISC-V, targeting
//! Tenstorrent hardware acceleration.
//!
//! ## Feature Flags
//!
//! - `riscv-zkn`: Use RISC-V Zkn scalar crypto instructions (AES, SHA)
//! - `riscv-zvk`: Use RISC-V Zvk* vector crypto instructions
//! - `tenstorrent`: Offload to TT-Metalium kernels on Tensix hardware
//!
//! ## Supported Primitives
//!
//! - Keccak-256 (Ethereum's hash)
//! - SHA-256 (Bitcoin, Merkle trees)
//! - ECDSA secp256k1 (Ethereum transaction signing) — planned
//! - NTT/iNTT (ZK proof generation) — planned
//! - Polynomial multiplication (FHE bootstrapping) — planned

#![cfg_attr(not(feature = "std"), no_std)]

pub mod keccak;
pub mod sha256;

/// Re-export hash functions at crate root
pub use keccak::keccak256;
pub use sha256::sha256;
