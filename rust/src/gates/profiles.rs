use crate::KeyValueOwned as KV;

pub const ARITHMETIC_CIRCUIT: &str = "arithmetic_circuit";

pub fn config_for_profile_arithmetic() -> Vec<KV> {
    vec![
        KV::from(("gate", "constant")),
        KV::from(("gate", "instance_var")),
        KV::from(("gate", "witness")),
        KV::from(("gate", "assert_zero")),
        KV::from(("gate", "add")),
        KV::from(("gate", "mul")),
    ]
}