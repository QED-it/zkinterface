use crate::{ConstraintSystemOwned, GateSystemOwned, GateOwned, CircuitHeaderOwned};
use GateOwned::*;
use crate::reading::Variable;
use super::profiles::{config_for_profile_arithmetic, ARITHMETIC_CIRCUIT};
use super::builder::{IBuilder, CachingBuilder};
use crate::gates::profiles::switch_profile;
use crate::gates::builder::OptimizingBuilder;


pub fn r1cs_to_gates(
    header: &CircuitHeaderOwned,
    r1cs: &ConstraintSystemOwned,
) -> (CircuitHeaderOwned, GateSystemOwned) {
    let mut bb = CachingBuilder::default();

    allocate_r1cs_variables(header, &mut bb);

    let mut bb = OptimizingBuilder::new(bb);
    let b = &mut bb;

    // Allocate negative one for negation.
    let neg_one = b.gate(Constant(0, header.field_maximum.clone().unwrap()));

    // Convert each R1CS constraint into a graph of Add/Mul/AssertZero gates.
    for constraint in &r1cs.constraints {
        let sum_a = build_lc(b, &constraint.linear_combination_a.get_variables());
        let sum_b = build_lc(b, &constraint.linear_combination_b.get_variables());
        let sum_c = build_lc(b, &constraint.linear_combination_c.get_variables());

        let prod = b.gate(Mul(0, sum_a, sum_b));
        let neg_c = b.gate(Mul(0, neg_one, sum_c));
        let claim_zero = b.gate(Add(0, prod, neg_c));
        b.gate(AssertZero(claim_zero));
    }

    let header = CircuitHeaderOwned {
        instance_variables: header.instance_variables.clone(),
        free_variable_id: b.free_id(),
        field_maximum: header.field_maximum.clone(),
        configuration: Some(switch_profile(
            &header.configuration,
            config_for_profile_arithmetic())),
        profile_name: Some(ARITHMETIC_CIRCUIT.to_string()),
    };

    (header, bb.builder.builder.gate_system)
}

fn allocate_r1cs_variables(header: &CircuitHeaderOwned, b: &mut CachingBuilder) {
    // Allocate the constant one of R1CS.
    let _one_id = b.gate(Constant(0, vec![1]));
    assert_eq!(_one_id, 0);

    // Allocate instance variables.
    for i in &header.instance_variables.variable_ids {
        let j = b.gate(InstanceVar(0));
        assert_eq!(*i, j, "Only consecutive instance variable IDs are supported.");
    }

    // Allocate witness variables.
    for i in b.free_id()..header.free_variable_id {
        let j = b.gate(Witness(0));
        assert_eq!(i, j);
    }
}


fn build_lc(
    b: &mut impl IBuilder,
    lc: &Vec<Variable>,
) -> u64 {
    if lc.len() == 0 {
        return b.gate(Constant(0, vec![]));
    }

    let mut sum_id = build_term(b, &lc[0]);

    for term in &lc[1..] {
        let term_id = build_term(b, term);
        sum_id = b.gate(Add(0, sum_id, term_id));
    }

    sum_id
}

fn build_term(
    b: &mut impl IBuilder,
    term: &Variable,
) -> u64 {
    let c = b.gate(Constant(0, term.value.to_vec()));
    b.gate(Mul(0, c, term.id))
}

#[test]
fn test_r1cs_to_gates() {
    use crate::examples::*;

    let r1cs_header = example_circuit_header();
    let witness = example_witness();
    let r1cs_system = example_constraints();

    let (gate_header, gate_system) = r1cs_to_gates(&r1cs_header, &r1cs_system);

    eprintln!();
    eprintln!("{}", gate_header);
    eprintln!("{}", witness);
    eprintln!("{}", gate_system);
}