use crate::{Result, KeyValueOwned as KV, CircuitHeaderOwned};

pub const ARITHMETIC_CIRCUIT: &str = "arithmetic_circuit";
pub const GATE: &str = "Gate";


pub fn config_for_profile_arithmetic() -> Vec<KV> {
    vec![
        KV::from((GATE, "Constant")),
        KV::from((GATE, "InstanceVar")),
        KV::from((GATE, "Witness")),
        KV::from((GATE, "AssertZero")),
        KV::from((GATE, "Add")),
        KV::from((GATE, "Mul")),
    ]
}

pub fn ensure_arithmetic_circuit_gate_type(gate_type: &str) -> Result<()> {
    match gate_type {
        "Constant" | "InstanceVar" | "Witness" | "AssertZero" | "Add" | "Mul" => Ok(()),
        _ => Err(format!("Gate type '{}' is not supported.", gate_type).into()),
    }
}

pub fn switch_profile(old_config: &Option<Vec<KV>>, mut profile_config: Vec<KV>) -> Vec<KV> {
    if let Some(old_config) = old_config.as_ref() {
        for kv in old_config {
            if kv.key != GATE {
                profile_config.push(kv.clone());
            }
        }
    }
    profile_config
}

pub fn ensure_arithmetic_circuit_profile(header: &CircuitHeaderOwned) -> Result<()> {
    // Check the selected profile.
    if header.profile_name != Some(ARITHMETIC_CIRCUIT.to_string()) {
        return Err(format!("Expected profile '{}'.", ARITHMETIC_CIRCUIT).into());
    }

    // Check the gate set.
    if let Some(config) = &header.configuration {
        for kv in config {
            if &kv.key == GATE {
                let gate_type: &str = kv.text.as_ref().ok_or("config gate should have a textual value.")?;
                ensure_arithmetic_circuit_gate_type(gate_type)?;
            }
        }
    }

    Ok(())
}
