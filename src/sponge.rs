use ark_ff::Field;

pub trait Permutation<F : Field, const N: usize> {
    fn apply(state: &mut [F; N]);
}



pub struct Sponge<F: Field, P: Permutation<F,N>, const RATE: usize, const N: usize> {
    state: [F; N],
    _marker: std::marker::PhantomData<P>,
}

impl<F:Field, P: Permutation<F, N>, const RATE: usize, const N: usize> Sponge<F, P, RATE, N> {
    pub fn new(start_state: [F; N]) -> Self {
        assert!(RATE <= N, "RATE must be less than or equal to N");
        Sponge::<F, P, RATE, N> {
            state: start_state,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn absorb(&mut self, input: &[F; RATE]) {
        for i in 0..RATE {
            self.state[i] += input[i];
        }
        P::apply(&mut self.state);
    }

    pub fn squeeze(&mut self) -> [F; RATE] {
        let output = unsafe {*(self.state[..RATE].as_ptr() as *const [F; RATE])}; 
        P::apply(&mut self.state);
        output
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_ff::{Zero, One};

    struct IdentityPermutation;
    impl<const N: usize> Permutation<Fr,{N}> for IdentityPermutation {
        fn apply(state: &mut [Fr; N]) {
        }
    }

    struct SimplePermutation;
    impl<const N: usize> Permutation<Fr,{N}> for SimplePermutation {
        fn apply(state: &mut [Fr; N]) {
            let first = state[0];
            for i in 0..(N-1) {
                state[i] = state[i+1];
            }
            state[N-1] = first;
        }
    }

    #[test]
    fn test_absorb() {
        let mut sponge = Sponge::<Fr, IdentityPermutation, 2, 4>::new([Fr::zero(); 4]);
        let input = [Fr::from(0x01), Fr::from(0x02)];
        sponge.absorb(&input);
        assert_eq!(sponge.state, [Fr::from(0x01), Fr::from(0x02), Fr::from(0x00), Fr::from(0x00)]);
        let mut sponge = Sponge::<Fr, IdentityPermutation, 3, 7>::new([Fr::zero(); 7]);
        let input = [Fr::from(0x01), Fr::from(0x02), Fr::from(0x03)];
        sponge.absorb(&input);
        assert_eq!(sponge.state, [Fr::from(0x01), Fr::from(0x02), Fr::from(0x03), Fr::from(0x00), Fr::from(0x00), Fr::from(0x00), Fr::from(0x00)]);
    }

    #[test]
    fn test_absorb_with_permutation() {
        let mut sponge = Sponge::<Fr, SimplePermutation, 2, 4>::new([Fr::zero(); 4]);
        let input = [Fr::from(0x01), Fr::from(0x02)];
        sponge.absorb(&input);
        assert_eq!(sponge.state, [Fr::from(0x02), Fr::from(0x00), Fr::from(0x00), Fr::from(0x01)]);
    }

    #[test]
    fn test_multiple_absorb() {
        let mut sponge = Sponge::<Fr, SimplePermutation, 2, 4>::new([Fr::zero(); 4]);
        let input1 = [Fr::from(0x01), Fr::from(0x02)];
        let input2 = [Fr::from(0x01), Fr::from(0x04)];
        sponge.absorb(&input1);
        sponge.absorb(&input2);
        assert_eq!(sponge.state, [Fr::from(0x04), Fr::from(0x00), Fr::from(0x01), Fr::from(0x03)]);
    }

    #[test]
    fn test_squeeze() {
        let mut sponge = Sponge::<Fr, IdentityPermutation, 2, 4>::new([Fr::zero(); 4]);
        let input = [Fr::from(0x01), Fr::from(0x02)];
        sponge.absorb(&input);
        let output = sponge.squeeze();
        assert_eq!(output, [Fr::from(0x01), Fr::from(0x02)]);
    }

    #[test]
    fn test_multiple_squeeze() {
        let mut sponge = Sponge::<Fr, SimplePermutation, 2, 4>::new([Fr::from(0x01), Fr::from(0x02), Fr::from(0x03), Fr::from(0x04)]);
        let input = [Fr::from(0x00), Fr::from(0x00)];
        sponge.absorb(&input);
        let output1 = sponge.squeeze();
        let output2 = sponge.squeeze();
        assert_eq!(output1, [Fr::from(0x02), Fr::from(0x03)]);
        assert_eq!(output2, [Fr::from(0x03), Fr::from(0x04)]);
    }

    #[test]
    #[should_panic]
    fn test_rate_should_not_be_larger_than_n() {
        let mut sponge = Sponge::<Fr, SimplePermutation, 5, 4>::new([Fr::from(0x01), Fr::from(0x02), Fr::from(0x03), Fr::from(0x04)]);
    }
}

