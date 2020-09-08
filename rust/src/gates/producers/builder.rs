use crate::{GateOwned, GateSystemOwned};
use std::collections::HashMap;
use crate::gates::gates::GateOwned::Constant;

pub trait IBuilder {
    fn alloc(&mut self) -> u64;
    fn free_id(&self) -> u64;
    fn push_gate(&mut self, allocated_gate: GateOwned);

    fn create_gate(&mut self, non_allocated_gate: GateOwned) -> u64 {
        if !non_allocated_gate.has_output() {
            self.push_gate(non_allocated_gate);
            return 0;
        }
        assert_eq!(non_allocated_gate.get_output(), 0);
        let new_id = self.alloc();
        let allocated_gate = non_allocated_gate.with_output(new_id);
        self.push_gate(allocated_gate);
        new_id
    }
}

#[derive(Default)]
pub struct Builder {
    pub gate_system: GateSystemOwned,
    free_id: u64,
}

impl IBuilder for Builder {
    fn alloc(&mut self) -> u64 {
        let id = self.free_id;
        self.free_id += 1;
        id
    }

    fn free_id(&self) -> u64 {
        self.free_id
    }

    fn push_gate(&mut self, gate: GateOwned) {
        self.gate_system.gates.push(gate);
    }
}

#[derive(Default)]
pub struct CachingBuilder {
    pub builder: Builder,
    cache: HashMap<GateOwned, u64>,
}

impl IBuilder for CachingBuilder {
    fn alloc(&mut self) -> u64 {
        self.builder.alloc()
    }

    fn free_id(&self) -> u64 {
        self.builder.free_id()
    }

    fn push_gate(&mut self, allocated_gate: GateOwned) {
        self.builder.push_gate(allocated_gate)
    }

    fn create_gate(&mut self, gate: GateOwned) -> u64 {
        if gate.cacheable() {
            match self.cache.get(&gate) {
                Some(cached) => *cached,
                None => {
                    let id = self.builder.create_gate(gate.clone());
                    self.cache.insert(gate, id);
                    id
                }
            }
        } else {
            self.builder.create_gate(gate)
        }
    }
}


#[derive(Default)]
pub struct OptimizingBuilder {
    pub builder: CachingBuilder,
    zero: u64,
    one: u64,
}

impl OptimizingBuilder {
    pub fn new(mut builder: CachingBuilder) -> OptimizingBuilder {
        let zero = builder.create_gate(Constant(0, vec![]));
        let one = builder.create_gate(Constant(0, vec![1]));
        OptimizingBuilder { builder, zero, one }
    }
}

impl IBuilder for OptimizingBuilder {
    fn alloc(&mut self) -> u64 {
        self.builder.alloc()
    }

    fn free_id(&self) -> u64 {
        self.builder.free_id()
    }

    fn push_gate(&mut self, allocated_gate: GateOwned) {
        self.builder.push_gate(allocated_gate)
    }

    fn create_gate(&mut self, gate: GateOwned) -> u64 {
        match gate {
            GateOwned::Add(_, l, r) if l == self.zero => r,
            GateOwned::Add(_, l, r) if r == self.zero => l,
            GateOwned::Mul(_, l, r) if l == self.one => r,
            GateOwned::Mul(_, l, r) if r == self.one => l,
            _ => self.builder.create_gate(gate),
        }
    }
}
