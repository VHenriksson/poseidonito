//! This module implements a general Poseidon permutation function. Given a concrete
//! Poseidon configuration, the `perm` function applies the Poseidon permutation.

use ark_ff::Field;


use crate::configurations::poseidon_config::PoseidonConfig;

/// A helper function to perform matrix-vector multiplication.
/// This could probably be optimized.
fn matrix_vector_mul<F: Field, const T: usize>(matrix: &[[F; T]; T], vector: &[F; T]) -> [F; T] {
    let mut result = [F::zero(); T];
    for i in 0..T {
        for j in 0..T {
            result[i] += matrix[i][j] * vector[j];
        }
    }
    result
}


/// Performs the Poseidon permutation on the input words.
pub fn perm<F: Field, const T: usize, P: PoseidonConfig<F,T>>(input_words: &mut [F; T]) {
    let R_f = P::R_F/ 2;
    let mut round_constants_counter = 0;

    for _ in 0..R_f {
        for i in 0..T {
            input_words[i] = input_words[i] + P::round_constants()[round_constants_counter];
            round_constants_counter += 1;
        }
        for i in 0..T {
            input_words[i] = P::sbox(&input_words[i]);
        }
        *input_words = matrix_vector_mul(&P::mds_matrix(), &input_words);
    }

    for _ in 0..P::R_P {
        for i in 0..T {
            input_words[i] = input_words[i] + P::round_constants()[round_constants_counter];
            round_constants_counter += 1;
        }
        input_words[0] = P::sbox(&input_words[0]);
        *input_words = matrix_vector_mul(&P::mds_matrix(), &input_words);
    }

    for _ in 0..R_f {
        for i in 0..T {
            input_words[i] = input_words[i] + P::round_constants()[round_constants_counter];
            round_constants_counter += 1;
        }
        for i in 0..T {
            input_words[i] = P::sbox(&input_words[i]);
        }
        *input_words = matrix_vector_mul(&P::mds_matrix(), &input_words);
    }
}



#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;
    use ark_bn254::Fr;
    use ark_ff::{Zero, One};
    use once_cell::sync::Lazy;
    use crate::configurations::config_x5_254_3::ConfigX5_254_3;
    use crate::configurations::config_creation_helper_functions::parse_constants;


    struct IdentityConfig;
    impl PoseidonConfig<Fr, 3> for IdentityConfig {
        const R_F: usize = 2;
        const R_P: usize = 3;

        fn mds_matrix() -> &'static [[Fr; 3]; 3] {
            static IDENTITY_MATRIX: Lazy<[[Fr; 3]; 3]> = Lazy::new(|| {
                [[Fr::one(), Fr::zero(), Fr::zero()],
                 [Fr::zero(), Fr::one(), Fr::zero()],
                 [Fr::zero(), Fr::zero(), Fr::one()]]
            });
            &IDENTITY_MATRIX
        }

        fn round_constants() -> &'static [Fr] {
            static ZEROES: Lazy<Vec<Fr>> = Lazy::new(|| {
                vec![Fr::zero(); 3*(2 + 3)]
            });
            &ZEROES
        }

        fn sbox(x: &Fr) -> Fr {
            *x
        }
    }

    #[test]
    fn test_identity() {
        let mut input = [Fr::one(), Fr::zero(), Fr::zero()];
        let expected = input.clone();
        perm::<Fr, 3, IdentityConfig>(&mut input);
        assert_eq!(input, expected);

        let mut input = [Fr::from(32543)*Fr::one(), Fr::from(865324)*Fr::one(), Fr::from(987676534)*Fr::one()];
        let expected = input.clone();
        perm::<Fr, 3, IdentityConfig>(&mut input);
        assert_eq!(input, expected);
    }

    struct SboxConfig;
    impl PoseidonConfig<Fr, 3> for SboxConfig {
        const R_F: usize = 2;
        const R_P: usize = 3;

        fn mds_matrix() -> &'static [[Fr; 3]; 3] {
            static IDENTITY_MATRIX: Lazy<[[Fr; 3]; 3]> = Lazy::new(|| {
                [[Fr::one(), Fr::zero(), Fr::zero()],
                 [Fr::zero(), Fr::one(), Fr::zero()],
                 [Fr::zero(), Fr::zero(), Fr::one()]]
            });
            &IDENTITY_MATRIX
        }


        fn round_constants() -> &'static [Fr] {
            static ZEROES: Lazy<Vec<Fr>> = Lazy::new(|| {
                vec![Fr::zero(); 3*(2 + 3)]
            });
            &ZEROES
        }

        fn sbox(x: &Fr) -> Fr {
            x.pow([5])
        }
    }

    #[test]
    fn test_sbox() {
        let first = Fr::from(2);
        let second = Fr::from(3);
        let third = Fr::from(4);
        let mut input = [first, second, third];
        let expected = [first.pow([5*5*5*5*5]), second.pow([5*5]), third.pow([5*5])];
        perm::<Fr, 3, SboxConfig>(&mut input);
        assert_eq!(input, expected);
    }

    struct RoundConstantConfig;
    impl PoseidonConfig<Fr, 3> for RoundConstantConfig {
        const R_F: usize = 2;
        const R_P: usize = 3;

        fn mds_matrix() -> &'static [[Fr; 3]; 3] {
            static IDENTITY_MATRIX: Lazy<[[Fr; 3]; 3]> = Lazy::new(|| {
                [[Fr::one(), Fr::zero(), Fr::zero()],
                 [Fr::zero(), Fr::one(), Fr::zero()],
                 [Fr::zero(), Fr::zero(), Fr::one()]]
            });
            &IDENTITY_MATRIX
        }

        fn round_constants() -> &'static [Fr] {
            static ROUND_CONSTANTS: Lazy<Vec<Fr>> = Lazy::new(|| {
                vec![Fr::from(1), Fr::from(10), Fr::from(100),
                    Fr::from(2), Fr::from(20), Fr::from(200),
                    Fr::from(3), Fr::from(30), Fr::from(300),
                    Fr::from(4), Fr::from(40), Fr::from(400),
                    Fr::from(5), Fr::from(50), Fr::from(500)]
            });
            &ROUND_CONSTANTS
        }

        fn sbox(x: &Fr) -> Fr {
            *x
        }
    }

    #[test]
    fn test_round_constant() {
        let mut input = [Fr::one(), Fr::one(), Fr::one()];
        let expected = [Fr::from(16), Fr::from(151), Fr::from(1501)];
        perm::<Fr, 3, RoundConstantConfig>(&mut input);
        assert_eq!(input, expected);
    }

    struct MatrixConfig;
    impl PoseidonConfig<Fr, 3> for MatrixConfig {
        const R_F: usize = 2;
        const R_P: usize = 3;

        fn mds_matrix() -> &'static [[Fr; 3]; 3] {
            static MDS_MATRIX: Lazy<[[Fr; 3]; 3]> = Lazy::new(|| {
                [[Fr::zero(), Fr::one(), Fr::zero()],
                 [Fr::zero(), Fr::zero(), Fr::one()],
                 [Fr::from(2), Fr::zero(), Fr::zero()]]
            });
            &MDS_MATRIX
        }

        fn round_constants() -> &'static [Fr] {
            static ZEROES: Lazy<Vec<Fr>> = Lazy::new(|| {
                vec![Fr::zero(); 3*(2 + 3)]
            });
            &ZEROES
        }

        fn sbox(x: &Fr) -> Fr {
            *x
        }
    }

    #[test]
    fn test_matrix() {
        let mut input = [Fr::one(), Fr::from(10), Fr::from(100)];
        let expected = [Fr::from(200), Fr::from(4), Fr::from(40)];
        perm::<Fr, 3, MatrixConfig>(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_x5_254_3() {
        let mut input = [Fr::from(0), Fr::from(1), Fr::from(2)];
        let expected = parse_constants::<Fr>("115cc0f5e7d690413df64c6b9662e9cf2a3617f2743245519e19607a4417189a\n0fca49b798923ab0239de1c9e7a4a9a2210312b6a2f616d18b5a87f9b628ae29\n0e7ae82e40091e63cbd4f16a6d16310b3729d4b6e138fcf54110e2867045a30c");
        perm::<Fr, 3, ConfigX5_254_3>(&mut input);
        assert_eq!(input.len(), expected.len());
        for i in 0..input.len() {
            assert_eq!(input[i], expected[i], "Mismatch at index {}", i);
        }
    }
}