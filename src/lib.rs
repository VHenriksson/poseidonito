mod configurations;
mod sponge;
mod permutation;

pub mod x5_254_3 {
    use ark_ff::{Zero};
    use ark_bn254::Fr;
    use crate::configurations::config_x5_254_3::ConfigX5_254_3;
    use crate::permutation::perm;
    use crate::sponge::{Permutation, Sponge};

    struct Perm_x5_254_3;
    impl Permutation<Fr, 3> for Perm_x5_254_3 {
        fn apply(state: &mut [Fr; 3]) {
            perm::<Fr, 3, ConfigX5_254_3>(state);
        }
    }
    
    pub fn hash(input: &[ark_bn254::Fr]) -> [ark_bn254::Fr; 1] {
        let mut sponge = Sponge::<Fr, Perm_x5_254_3, 1, 3>::new([Fr::zero(); 3]);
        for i in input {
            sponge.absorb(&[*i]);
        }
        sponge.squeeze()
    }
}