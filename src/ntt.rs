//! Number Theoretic Transform for 256-coefficient polynomials over Zq.
//!
//! This module uses Kyber's prime modulus `q = 3329` and primitive
//! 256th root `17`. The multiplication helper implements cyclic
//! convolution in `Zq[x] / (x^256 - 1)`.

pub const N: usize = 256;
pub const Q: u16 = 3329;
pub const ROOT: u16 = 17;
pub const ROOT_INV: u16 = 1175;
pub const N_INV: u16 = 3316;

pub type Polynomial = [u16; N];

const fn mod_pow_const(base: u16, mut exp: u32) -> u16 {
    let mut acc = 1u32;
    let modulus = Q as u32;
    let mut b = base as u32;

    while exp > 0 {
        if exp & 1 == 1 {
            acc = (acc * b) % modulus;
        }
        b = (b * b) % modulus;
        exp >>= 1;
    }

    acc as u16
}

#[inline]
pub fn reduce(value: u32) -> u16 {
    (value % Q as u32) as u16
}

#[inline]
pub fn add_mod(a: u16, b: u16) -> u16 {
    reduce(a as u32 + b as u32)
}

#[inline]
pub fn sub_mod(a: u16, b: u16) -> u16 {
    if a >= b {
        a - b
    } else {
        (a as u32 + Q as u32 - b as u32) as u16
    }
}

#[inline]
pub fn mul_mod(a: u16, b: u16) -> u16 {
    reduce(a as u32 * b as u32)
}

fn bit_reverse_permute(poly: &mut Polynomial) {
    let mut j = 0usize;

    for i in 1..N {
        let mut bit = N >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;

        if i < j {
            poly.swap(i, j);
        }
    }
}

fn transform(poly: &mut Polynomial, root: u16) {
    bit_reverse_permute(poly);

    let mut len = 2usize;
    while len <= N {
        let stage_root = mod_pow_const(root, (N / len) as u32);

        let mut start = 0usize;
        while start < N {
            let mut w = 1u16;
            let half = len / 2;

            for offset in 0..half {
                let even = poly[start + offset];
                let odd = mul_mod(poly[start + offset + half], w);

                poly[start + offset] = add_mod(even, odd);
                poly[start + offset + half] = sub_mod(even, odd);
                w = mul_mod(w, stage_root);
            }

            start += len;
        }

        len <<= 1;
    }
}

/// Convert a coefficient-order polynomial into NTT evaluation form.
pub fn forward_ntt(poly: &mut Polynomial) {
    for coeff in poly.iter_mut() {
        *coeff = reduce(*coeff as u32);
    }
    transform(poly, ROOT);
}

/// Convert an NTT-form polynomial back to coefficient order.
pub fn inverse_ntt(poly: &mut Polynomial) {
    transform(poly, ROOT_INV);
    for coeff in poly.iter_mut() {
        *coeff = mul_mod(*coeff, N_INV);
    }
}

/// Multiply two 256-coefficient polynomials using an NTT and pointwise products.
pub fn multiply_polynomials(lhs: &Polynomial, rhs: &Polynomial) -> Polynomial {
    let mut lhs_ntt = *lhs;
    let mut rhs_ntt = *rhs;

    forward_ntt(&mut lhs_ntt);
    forward_ntt(&mut rhs_ntt);

    for i in 0..N {
        lhs_ntt[i] = mul_mod(lhs_ntt[i], rhs_ntt[i]);
    }

    inverse_ntt(&mut lhs_ntt);
    lhs_ntt
}

/// Baseline O(n^2) cyclic polynomial multiplication for tests and benchmarks.
pub fn multiply_polynomials_naive(lhs: &Polynomial, rhs: &Polynomial) -> Polynomial {
    let mut out = [0u16; N];

    for i in 0..N {
        for j in 0..N {
            let idx = (i + j) & (N - 1);
            let term = mul_mod(lhs[i], rhs[j]);
            out[idx] = add_mod(out[idx], term);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patterned_poly(seed: u16) -> Polynomial {
        let mut poly = [0u16; N];
        for (i, coeff) in poly.iter_mut().enumerate() {
            let x = i as u32;
            *coeff = reduce(seed as u32 + x * x * 17 + x * 31);
        }
        poly
    }

    fn centered(value: u16) -> i16 {
        if value > Q / 2 {
            (value as i32 - Q as i32) as i16
        } else {
            value as i16
        }
    }

    #[test]
    fn kyber_root_constants_match_reference_properties() {
        assert_eq!(Q, 3329);
        assert_eq!(ROOT, 17);
        assert_eq!(mod_pow_const(ROOT, 256), 1);
        assert_eq!(mod_pow_const(ROOT, 128), Q - 1);
        assert_eq!(mul_mod(ROOT, ROOT_INV), 1);
        assert_eq!(mul_mod(N as u16, N_INV), 1);
    }

    #[test]
    fn kyber_reference_zetas_match_root_generation_prefix() {
        const MONT: u16 = 2285; // 2^16 mod 3329
        const KYBER_TREE_PREFIX: [u8; 16] = [
            0, 64, 32, 96, 16, 80, 48, 112, 8, 72, 40, 104, 24, 88, 56, 120,
        ];
        const KYBER_ZETAS_PREFIX: [i16; 16] = [
            -1044, -758, -359, -1517, 1493, 1422, 287, 202, -171, 622, 1577, 182, 962, -1202,
            -1474, 1468,
        ];

        for (idx, tree_idx) in KYBER_TREE_PREFIX.iter().enumerate() {
            let generated = mul_mod(MONT, mod_pow_const(ROOT, *tree_idx as u32));
            assert_eq!(centered(generated), KYBER_ZETAS_PREFIX[idx]);
        }
    }

    #[test]
    fn ntt_round_trips_zero_one_and_patterned_polynomials() {
        let cases = [
            [0u16; N],
            {
                let mut one = [0u16; N];
                one[0] = 1;
                one
            },
            patterned_poly(7),
        ];

        for original in cases {
            let mut transformed = original;
            forward_ntt(&mut transformed);
            inverse_ntt(&mut transformed);
            assert_eq!(transformed, original);
        }
    }

    #[test]
    fn ntt_multiply_matches_naive_cyclic_convolution() {
        let lhs = patterned_poly(3);
        let rhs = patterned_poly(41);

        assert_eq!(
            multiply_polynomials(&lhs, &rhs),
            multiply_polynomials_naive(&lhs, &rhs)
        );
    }

    #[test]
    fn ntt_multiply_handles_sparse_basis_terms() {
        let mut lhs = [0u16; N];
        let mut rhs = [0u16; N];
        lhs[255] = 7;
        rhs[3] = 11;

        let product = multiply_polynomials(&lhs, &rhs);

        assert_eq!(product[2], mul_mod(7, 11));
        for (idx, coeff) in product.iter().enumerate() {
            if idx != 2 {
                assert_eq!(*coeff, 0);
            }
        }
    }
}
