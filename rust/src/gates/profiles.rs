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

pub fn switch_profile(old_config: &Option<Vec<KV>>, mut profile_config: Vec<KV>) -> Vec<KV> {
    if let Some(old_config) = old_config.as_ref() {
        for kv in old_config {
            if kv.key != "gate" {
                profile_config.push(kv.clone());
            }
        }
    }
    profile_config
}