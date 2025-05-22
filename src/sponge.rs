

pub trait Permutation<const N: usize> {
    fn apply(state: &mut [u64; N]);
}



pub struct Sponge<P: Permutation<N>, const RATE: usize, const N: usize> {
    state: [u64; N],
    _marker: std::marker::PhantomData<P>,
}

impl<P: Permutation<N>, const RATE: usize, const N: usize> Sponge<P, RATE, N> {
    pub fn new(start_state: [u64; N]) -> Self {
        assert!(RATE <= N, "RATE must be less than or equal to N");
        Sponge::<P, RATE, N> {
            state: start_state,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn absorb(&mut self, input: &[u64; RATE]) {
        for i in 0..RATE {
            self.state[i] ^= input[i];
        }
        P::apply(&mut self.state);
    }

    pub fn squeeze(&mut self) -> [u64; RATE] {
        // TODO: For the unsafe to be safe, we need to ensure that RATE is smaller than N
        let output = unsafe {*(self.state[..RATE].as_ptr() as *const [u64; RATE])}; 
        P::apply(&mut self.state);
        output
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    struct IdentityPermutation;
    impl<const N: usize> Permutation<{N}> for IdentityPermutation {
        fn apply(state: &mut [u64; N]) {
        }
    }

    struct SimplePermutation;
    impl<const N: usize> Permutation<{N}> for SimplePermutation {
        fn apply(state: &mut [u64; N]) {
            let first = state[0];
            for i in 0..(N-1) {
                state[i] = state[i+1];
            }
            state[N-1] = first;
        }
    }

    #[test]
    fn test_absorb() {
        let mut sponge = Sponge::<IdentityPermutation, 2, 4>::new([0; 4]);
        let input = [0x01, 0x02];
        sponge.absorb(&input);
        assert_eq!(sponge.state, [0x01, 0x02, 0x00, 0x00]);
        let mut sponge = Sponge::<IdentityPermutation, 3, 7>::new([0; 7]);
        let input = [0x01, 0x02, 0x03];
        sponge.absorb(&input);
        assert_eq!(sponge.state, [0x01, 0x02, 0x03, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_absorb_with_permutation() {
        let mut sponge = Sponge::<SimplePermutation, 2, 4>::new([0; 4]);
        let input = [0x01, 0x02];
        sponge.absorb(&input);
        assert_eq!(sponge.state, [0x02, 0x00, 0x00, 0x01]);
    }

    #[test]
    fn test_multiple_absorb() {
        let mut sponge = Sponge::<SimplePermutation, 2, 4>::new([0; 4]);
        let input1 = [0x01, 0x02];
        let input2 = [0x01, 0x04];
        sponge.absorb(&input1);
        sponge.absorb(&input2);
        assert_eq!(sponge.state, [0x04, 0x00, 0x01, 0x03]);
    }

    #[test]
    fn test_squeeze() {
        let mut sponge = Sponge::<IdentityPermutation, 2, 4>::new([0; 4]);
        let input = [0x01, 0x02];
        sponge.absorb(&input);
        let output = sponge.squeeze();
        assert_eq!(output, [0x01, 0x02]);
    }

    #[test]
    fn test_multiple_squeeze() {
        let mut sponge = Sponge::<SimplePermutation, 2, 4>::new([0x01, 0x02, 0x03, 0x04]);
        let input = [0x00, 0x00];
        sponge.absorb(&input);
        let output1 = sponge.squeeze();
        let output2 = sponge.squeeze();
        assert_eq!(output1, [0x02, 0x03]);
        assert_eq!(output2, [0x03, 0x04]);
    }

    #[test]
    #[should_panic]
    fn test_rate_should_not_be_larger_than_n() {
        let mut sponge = Sponge::<SimplePermutation, 5, 4>::new([0x01, 0x02, 0x03, 0x04]);
    }
}

