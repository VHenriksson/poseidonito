use ark_ff::{Field, PrimeField, BigInteger};
use hex::{FromHex, encode};


/// Converts a hex string to a field element.
fn hex_to_field<F: Field>(hex_str: &str) -> Result<F, String> {
    if hex_str.trim().is_empty() {
        return Err("Empty hex string".to_string());
    }
    let bytes = Vec::<u8>::from_hex(hex_str).expect("Invalid hex");
    let bytes = bytes
        .into_iter()
        .rev()
        .collect::<Vec<u8>>();
    // Note that we really do want to panic here if the conversion fails,
    // as this indicates that the provided hex strings, which should always be
    // known at compile time, are not valid for the field F.
    Ok(F::from_random_bytes(&bytes).expect("Failed to convert hex to field element"))
}

/// Parses a string of hex-encoded field elements, one per line,
/// into a vector of field elements.
pub fn parse_constants<F: Field>(raw_constants: &str) -> Vec<F> {
    raw_constants
        .lines()
        .filter_map(|line| hex_to_field(line).ok())
        .collect()
}

/// Parses a matrix represented as an array of strings, where each string has `T` space-separated
/// hex-encoded field elements.
pub fn parse_matrix<F: Field, const T: usize>(rows: [&str; T]) -> [[F; T]; T] {
    let mut matrix = [[F::zero(); T]; T];
    for (i, row) in rows.iter().enumerate() {
        let elements: Vec<F> = row
            .split_whitespace()
            .filter_map(|s| hex_to_field(s).ok())
            .collect();
        if elements.len() != T {
            panic!("Row {} does not have the correct number of elements", i);
        }
        matrix[i].copy_from_slice(&elements);
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;

    #[test]
    fn test_parse_constants() {
        let constants = "0ee9a592ba9a9518d05986d656f40c2114c4993c11bb29938d21d47304cd8e6e\n00f1445235f2148c5986587169fc1bcd887b08d4d00868df5696fff40956e864";
        let elements = parse_constants::<Fr>(&constants);
        assert_eq!(elements.len(), 2);
        // By testing against the sum given as a hex string, we can test that parsing is correct,
        // without bothering about the specifics of the field representation.
        let sum: Fr = elements.iter().sum();
        let expected_sum = "0fdae9e4f08ca9a529dfdf47c0f027ee9d3fa210e1c39272e3b8d4670e2476d2";
        assert_eq!(expected_sum, encode(sum.into_bigint().to_bytes_be()));
    }

    #[test]
    fn test_leave_empty_rows() {
        let constants = "0ee9a592ba9a9518d05986d656f40c2114c4993c11bb29938d21d47304cd8e6e\n\n00f1445235f2148c5986587169fc1bcd887b08d4d00868df5696fff40956e864\n\n\n";
        let elements = parse_constants::<Fr>(&constants);
        assert_eq!(elements.len(), 2);
    
    }

    #[test]
    fn test_read_many() {
        let constants = include_str!("config_x5_254_3/constants.txt");
        let elements = parse_constants::<Fr>(constants);
        assert_eq!(elements.len(), 195);
    }

    #[test]
    fn test_parse_matrix() {
        let matrix_str = [
            "109b7f411ba0e4c9b2b70caf5c36a7b194be7c11ad24378bfedb68592ba8118b 16ed41e13bb9c0c66ae119424fddbcbc9314dc9fdbdeea55d6c64543dc4903e0 2b90bba00fca0589f617e7dcbfe82e0df706ab640ceb247b791a93b74e36736d",
            "2969f27eed31a480b9c36c764379dbca2cc8fdd1415c3dded62940bcde0bd771 2e2419f9ec02ec394c9871c832963dc1b89d743c8c7b964029b2311687b1fe23 101071f0032379b697315876690f053d148d4e109f5fb065c8aacc55a0f89bfa",
            "143021ec686a3f330d5f9e654638065ce6cd79e28c5b3753326244ee65a1b1a7 176cc029695ad02582a70eff08a6fd99d057e12e58e7d7b6b16cdfabc8ee2911 19a3fc0a56702bf417ba7fee3802593fa644470307043f7773279cd71d25d5e0"
        ];
        let matrix = parse_matrix::<Fr, 3>(matrix_str);
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0].len(), 3);
        assert_eq!(matrix[0][0], hex_to_field::<Fr>("109b7f411ba0e4c9b2b70caf5c36a7b194be7c11ad24378bfedb68592ba8118b").unwrap());
        assert_eq!(matrix[1][1], hex_to_field::<Fr>("2e2419f9ec02ec394c9871c832963dc1b89d743c8c7b964029b2311687b1fe23").unwrap());
        assert_eq!(matrix[2][2], hex_to_field::<Fr>("19a3fc0a56702bf417ba7fee3802593fa644470307043f7773279cd71d25d5e0").unwrap());
    }
}