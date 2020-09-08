use std::fmt;
use num_bigint::BigUint;
use colored::Colorize;
use crate::{GateSystemOwned, GateOwned, GateOwned::*, MessagesOwned, CircuitHeaderOwned, WitnessOwned, KeyValueOwned};


impl fmt::Display for MessagesOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for msg in &self.circuit_headers {
            f.write_fmt(format_args!("{}\n", msg))?;
        }
        for msg in &self.constraint_systems {
            f.write_fmt(format_args!("{:?}\n\n", msg))?;
        }
        for msg in &self.gate_systems {
            f.write_fmt(format_args!("{}\n", msg))?;
        }
        for msg in &self.witnesses {
            f.write_fmt(format_args!("{}\n", msg))?;
        }
        Ok(())
    }
}

impl fmt::Display for CircuitHeaderOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.free_variable_id > 0 {
            f.write_fmt(format_args!("{} {}\n", fmt_kw("FreeVariableId"), self.free_variable_id))?;
        }

        if let Some(ref field) = self.field_maximum {
            f.write_fmt(format_args!("{} {}\n", fmt_kw("FieldMaximum"), fmt_field(field)))?;
        }

        for kv in self.configuration.as_ref().unwrap() {
            f.write_fmt(format_args!("{}\n", kv))?;
        }

        if let Some(ref p) = self.profile_name {
            f.write_fmt(format_args!("{} {}\n", fmt_kw("Profile"), p))?;
        }

        if self.instance_variables.values.is_some() {
            for var in self.instance_variables.get_variables() {
                f.write_fmt(format_args!("{} {} = {}\n", fmt_kw("SetInstanceVar"), fmt_wire(var.id), fmt_field(var.value)))?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for GateSystemOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for gate in &self.gates {
            f.write_fmt(format_args!("{}\n", gate))?;
        }
        Ok(())
    }
}

impl fmt::Display for WitnessOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vars = self.assigned_variables.get_variables();
        for var in vars {
            f.write_fmt(format_args!("{}\t= {}\n", fmt_wire(var.id), fmt_field(var.value)))?;
        }
        Ok(())
    }
}

impl fmt::Display for GateOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant(output, constant) =>
                f.write_fmt(format_args!("{}\t<- {} {}", fmt_wire(*output), fmt_kw("Constant"), fmt_field(constant))),

            InstanceVar(output) =>
                f.write_fmt(format_args!("{}\t<- {}", fmt_wire(*output), fmt_kw("InstanceVar"))),

            Witness(output) =>
                f.write_fmt(format_args!("{}\t<- {}", fmt_wire(*output), fmt_kw("Witness"))),

            AssertZero(input) =>
                f.write_fmt(format_args!("{} {}", fmt_kw("AssertZero"), fmt_wire(*input))),

            Add(output, left, right) =>
                f.write_fmt(format_args!("{}\t<- {} {} {}", fmt_wire(*output), fmt_wire(*left), fmt_kw("+"), fmt_wire(*right))),

            Mul(output, left, right) =>
                f.write_fmt(format_args!("{}\t<- {} {} {}", fmt_wire(*output), fmt_wire(*left), fmt_kw("*"), fmt_wire(*right))),
        }
    }
}

impl fmt::Display for KeyValueOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref text) = self.text {
            f.write_fmt(format_args!("{} {}", fmt_kw(&self.key), text))
        } else if let Some(ref data) = self.data {
            f.write_fmt(format_args!("{} {:?}", fmt_kw(&self.key), data))
        } else {
            f.write_fmt(format_args!("{} {}", fmt_kw(&self.key), self.number))
        }
    }
}

pub fn fmt_field(val: &[u8]) -> String {
    BigUint::from_bytes_le(val).to_string().green().to_string()
}

pub fn fmt_wire(id: u64) -> String {
    format!("wire_{}", id).blue().to_string()
}

pub fn fmt_kw(s: &str) -> String {
    s.purple().bold().to_string()
}
