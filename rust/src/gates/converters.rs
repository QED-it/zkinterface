use crate::{ConstraintSystemOwned, GateSystemOwned, GateOwned, VariablesOwned};
use GateOwned::*;
use crate::reading::Variable;

pub trait IBuilder {
    fn alloc(&mut self) -> u64;
    fn push_gate(&mut self, allocated_gate: GateOwned);

    fn gate(&mut self, non_allocated_gate: GateOwned) -> u64 {
        assert_eq!(non_allocated_gate.get_output(), 0);
        let new_id = self.alloc();
        let allocated_gate = non_allocated_gate.with_output(new_id);
        self.push_gate(allocated_gate);
        new_id
    }
}

#[derive(Default)]
struct Builder {
    free_id: u64,
    gs: GateSystemOwned,
}

impl IBuilder for Builder {
    fn alloc(&mut self) -> u64 {
        let id = self.free_id;
        self.free_id += 1;
        id
    }

    fn push_gate(&mut self, gate: GateOwned) {
        self.gs.gates.push(gate);
    }
}


pub fn r1cs_to_gates(r1cs: &ConstraintSystemOwned) -> GateSystemOwned {
    let mut bb = Builder::default();
    let b = &mut bb;

    for constraint in &r1cs.constraints {
        let sum_a = build_lc(b, &constraint.linear_combination_a.get_variables());
        let sum_b = build_lc(b, &constraint.linear_combination_b.get_variables());
        let sum_c = build_lc(b, &constraint.linear_combination_c.get_variables());

        let prod = b.gate(Mul(0, sum_a, sum_b));
        //let sum_c = b.alloc_gate(Neg(0, sum_c));
        let claim_zero = b.gate(Add(0, prod, sum_c));
        b.gate(AssertZero(claim_zero));
    }

    bb.gs
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
    b.gate(Mul(0, term.id, c))
}