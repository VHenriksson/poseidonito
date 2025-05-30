mod configurations;
mod sponge;
mod permutation;

pub mod x5_254_3 {
    use ark_ff::{Zero};
    use ark_bn254::Fr;
    use crate::configurations::config_x5_254_3::ConfigX5_254_3;
    use crate::permutation::perm;

    struct Hades;
    impl crate::sponge::Permutation<Fr, 3> for Hades {
        fn apply(state: &mut [Fr; 3]) {
            perm::<Fr, 3, ConfigX5_254_3>(state);
        }
    }
    
    pub fn hash(input: &[ark_bn254::Fr]) -> [ark_bn254::Fr; 1] {
        let mut sponge = crate::sponge::Sponge::<ark_bn254::Fr, Hades, 1, 3>::new([ark_bn254::Fr::zero(); 3]);
        for i in input {
            sponge.absorb(&[*i]);
        }
        sponge.squeeze()
    }
}