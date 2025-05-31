
use ark_bn254::Fr;
use poseidonito::x5_254_3::hash;

fn main() {
    let input: Vec<Fr> = (1..100000).map(|x| Fr::from(x)).collect();
    let output = hash(&input);
    println!("Hash output: {:?}", output);
}