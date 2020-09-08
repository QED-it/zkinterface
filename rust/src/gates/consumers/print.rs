use num_bigint::BigUint;
use colored::Colorize;

pub fn fmt_field(val: &[u8]) -> String {
    BigUint::from_bytes_le(val).to_string().green().to_string()
}

pub fn fmt_wire(id: u64) -> String {
    format!("wire_{}", id).blue().to_string()
}

pub fn fmt_kw(s: &str) -> String {
    s.purple().bold().to_string()
}
