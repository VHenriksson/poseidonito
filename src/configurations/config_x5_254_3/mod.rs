//! The Poseidon configuration for the BN254 curve with 3 inputs and a 5-round sbox.
//! Obtained from the reference implementation at https://extgit.isec.tugraz.at/krypto/hadeshash

use ark_ff::Field;
use ark_bn254::Fr;
use once_cell::sync::Lazy;
use crate::configurations::poseidon_config::PoseidonConfig;
use crate::configurations::config_creation_helper_functions::{parse_constants, parse_matrix};

/// The Poseidon configuration for the BN254 curve with 3 inputs and a 5-round sbox.
pub struct ConfigX5_254_3;

static ROUND_CONSTANTS: Lazy<Vec<Fr>> = Lazy::new(|| {
    parse_constants(include_str!("constants.txt"))
});

static MDS_MATRIX: Lazy<[[Fr; 3]; 3]> = Lazy::new(|| {
    parse_matrix([
        "109b7f411ba0e4c9b2b70caf5c36a7b194be7c11ad24378bfedb68592ba8118b 16ed41e13bb9c0c66ae119424fddbcbc9314dc9fdbdeea55d6c64543dc4903e0 2b90bba00fca0589f617e7dcbfe82e0df706ab640ceb247b791a93b74e36736d",
        "2969f27eed31a480b9c36c764379dbca2cc8fdd1415c3dded62940bcde0bd771 2e2419f9ec02ec394c9871c832963dc1b89d743c8c7b964029b2311687b1fe23 101071f0032379b697315876690f053d148d4e109f5fb065c8aacc55a0f89bfa",
        "143021ec686a3f330d5f9e654638065ce6cd79e28c5b3753326244ee65a1b1a7 176cc029695ad02582a70eff08a6fd99d057e12e58e7d7b6b16cdfabc8ee2911 19a3fc0a56702bf417ba7fee3802593fa644470307043f7773279cd71d25d5e0"
    ])
});

impl PoseidonConfig<Fr, 3> for ConfigX5_254_3 {
    const R_F: usize = 8;
    const R_P: usize = 57;


    fn mds_matrix() -> &'static [[Fr; 3]; 3] {
        &MDS_MATRIX
    }

    fn round_constants() -> &'static [Fr] {
        &ROUND_CONSTANTS
    }

    fn sbox(x: &Fr) -> Fr {
        x.pow([5])
    }
}