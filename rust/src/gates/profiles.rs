use crate::KeyValueOwned as KV;

pub fn config_for_profile_arithmetic() -> Vec<KV> {
    vec![
        KV::from(("gate", "constant")),
        KV::from(("gate", "instance_var")),
        KV::from(("gate", "witness")),
        KV::from(("gate", "add")),
        KV::from(("gate", "mul")),
        KV::from(("gate", "assert_zero")),
    ]
}