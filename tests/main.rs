#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use num_bigint::BigInt;

    // Assuming the `rust_witness` crate is correctly linked and available
    use rust_witness::witness;

    #[test]
    fn test_multiplier2_witness() {
        // Prepare inputs
        let mut inputs = HashMap::new();
        let a = BigInt::from(10u8);
        let b = BigInt::from(20u8);
        inputs.insert("a".to_string(), vec![a]);
        inputs.insert("b".to_string(), vec![b]);

        println!("input: {:?}", inputs);

        witness!(multiplier2);

        let witness = {
            multiplier2_witness(inputs)
                .into_iter()
                .map(|w| w.to_biguint().unwrap())
                .collect::<Vec<_>>()
        };

        println!("witness: {:?}", witness);

        assert!(!witness.is_empty(), "Witness should not be empty");
    }
}
