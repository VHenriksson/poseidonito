use ark_ff::Field;

pub trait PoseidonConfig<F: Field, const T: usize> {

    const R_F: usize;

    const R_P: usize;

    fn mds_matrix() -> [[F; T]; T];

    fn round_constants() -> Vec<F>;

    fn sbox(x: &F) -> F;

}
