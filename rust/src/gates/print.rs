use num_bigint::BigUint;

pub fn value_to_string(val: &[u8]) -> String {
    BigUint::from_bytes_le(val).to_string()
}