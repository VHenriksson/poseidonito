//! This module defines the `PoseidonConfig` trait, which is used to configure concrete
//! Poseidon hash functions.
//! 
//! Any Poseidon hash function is defined by its parameters:
//! 
//! - `R_F`: The number of full rounds.
//! - `R_P`: The number of partial rounds.
//! - `mds_matrix`: The MDS matrix used in the permutation.
//! - `round_constants`: The round constants used in the permutation.
//! - `sbox`: The S-box function used in the permutation.


use ark_ff::Field;

/// To create a specific Poseidon hash function, one needs to implement the `PoseidonConfig` trait.
pub trait PoseidonConfig<F: Field, const T: usize> {

    /// The number of full rounds in the Poseidon permutation.
    const R_F: usize;

    /// The number of partial rounds in the Poseidon permutation.
    const R_P: usize;

    /// The MDS matrix used in the Poseidon permutation (the linear diffusion layer).
    fn mds_matrix() -> &'static [[F; T]; T];

    /// The round constants used in the Poseidon permutation.
    fn round_constants() -> &'static [F];

    /// The S-box function used in the Poseidon permutation.
    fn sbox(x: &F) -> F;

}
