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
        // We probably want something other than all zeroes as the initial state.
        // However, I have not yet found any reference value, so all zeroes will be
        // used as a placeholder for now.
        let mut sponge = Sponge::<Fr, Perm_x5_254_3, 1, 3>::new([Fr::zero(); 3]);
        for i in input {
            sponge.absorb(&[*i]);
        }
        sponge.squeeze()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_ff::{Zero, One};

    #[test]
    fn test_hash() {
        let input_1 = vec![Fr::from(1), Fr::from(2), Fr::from(3)];
        let result_1 = x5_254_3::hash(&input_1);
        let input_2 = vec![Fr::from(1), Fr::from(2), Fr::from(3), Fr::zero()];
        let result_2 = x5_254_3::hash(&input_2);
        let input_3 = vec![Fr::from(1), Fr::from(2), Fr::from(3)];
        let result_3 = x5_254_3::hash(&input_1);
        // Since I have not found any reference values yet, I will be happy with asserting
        // that equal inputs yield equal outputs, and different inputs yield different outputs.
        assert_ne!(result_1, result_2);
        assert_eq!(result_1, result_3);
    }
}