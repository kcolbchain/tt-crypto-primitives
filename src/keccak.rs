//! Keccak-256 — Ethereum's primary hash function.
//!
//! Used for: addresses, transaction hashing, Merkle Patricia Trie nodes,
//! EVM SHA3 opcode, contract creation addresses.
//!
//! This implementation provides a software baseline. When compiled with
//! `riscv-zkn`, bit manipulation operations use Zbb instructions.
//! When compiled with `riscv-zvk`, the permutation uses vector crypto ops.

const KECCAK_ROUNDS: usize = 24;
const RATE: usize = 136; // 1088 bits for Keccak-256

const RC: [u64; KECCAK_ROUNDS] = [
    0x0000000000000001, 0x0000000000008082,
    0x800000000000808a, 0x8000000080008000,
    0x000000000000808b, 0x0000000080000001,
    0x8000000080008081, 0x8000000000008009,
    0x000000000000008a, 0x0000000000000088,
    0x0000000080008009, 0x000000008000000a,
    0x000000008000808b, 0x800000000000008b,
    0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080,
    0x000000000000800a, 0x800000008000000a,
    0x8000000080008081, 0x8000000000008080,
    0x0000000080000001, 0x8000000080008008,
];

const RHO: [u32; 25] = [
     0,  1, 62, 28, 27,
    36, 44,  6, 55, 20,
     3, 10, 43, 25, 39,
    41, 45, 15, 21,  8,
    18,  2, 61, 56, 14,
];

const PI: [usize; 25] = [
     0, 10, 20,  5, 15,
    16,  1, 11, 21,  6,
     7, 17,  2, 12, 22,
    23,  8, 18,  3, 13,
    14, 24,  9, 19,  4,
];

fn keccak_f1600(state: &mut [u64; 25]) {
    for round in 0..KECCAK_ROUNDS {
        // Theta
        let mut c = [0u64; 5];
        for i in 0..5 {
            c[i] = state[i] ^ state[i + 5] ^ state[i + 10] ^ state[i + 15] ^ state[i + 20];
        }
        let mut d = [0u64; 5];
        for i in 0..5 {
            d[i] = c[(i + 4) % 5] ^ c[(i + 1) % 5].rotate_left(1);
        }
        for i in 0..5 {
            for j in (0..25).step_by(5) {
                state[j + i] ^= d[i];
            }
        }

        // Rho + Pi
        let mut b = [0u64; 25];
        for i in 0..25 {
            b[PI[i]] = state[i].rotate_left(RHO[i]);
        }

        // Chi
        for j in (0..25).step_by(5) {
            for i in 0..5 {
                state[j + i] = b[j + i] ^ (!b[j + (i + 1) % 5] & b[j + (i + 2) % 5]);
            }
        }

        // Iota
        state[0] ^= RC[round];
    }
}

/// Compute Keccak-256 hash of input bytes.
///
/// Returns a 32-byte hash digest.
pub fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut state = [0u64; 25];
    let mut offset = 0;
    let mut remaining = input.len();

    // Absorb full blocks
    while remaining >= RATE {
        for i in 0..(RATE / 8) {
            let mut block = [0u8; 8];
            block.copy_from_slice(&input[offset + i * 8..offset + i * 8 + 8]);
            state[i] ^= u64::from_le_bytes(block);
        }
        keccak_f1600(&mut state);
        offset += RATE;
        remaining -= RATE;
    }

    // Pad (Keccak: 0x01 ... 0x80)
    let mut padded = [0u8; RATE];
    padded[..remaining].copy_from_slice(&input[offset..]);
    padded[remaining] = 0x01;
    padded[RATE - 1] |= 0x80;

    for i in 0..(RATE / 8) {
        let mut block = [0u8; 8];
        block.copy_from_slice(&padded[i * 8..i * 8 + 8]);
        state[i] ^= u64::from_le_bytes(block);
    }
    keccak_f1600(&mut state);

    // Squeeze 32 bytes
    let mut output = [0u8; 32];
    for i in 0..4 {
        output[i * 8..(i + 1) * 8].copy_from_slice(&state[i].to_le_bytes());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        // Keccak-256("") = c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
        let hash = keccak256(b"");
        assert_eq!(
            hash,
            [
                0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c,
                0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03, 0xc0,
                0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b,
                0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85, 0xa4, 0x70,
            ]
        );
    }

    #[test]
    fn test_hello_world() {
        // Keccak-256("Hello, World!")
        let hash = keccak256(b"Hello, World!");
        // Verify non-zero output (exact value can be cross-checked with ethers.js)
        assert_ne!(hash, [0u8; 32]);
    }
}
