use crate::{GateOwned, GateSystemOwned};
use std::collections::HashMap;

pub trait IBuilder {
    fn alloc(&mut self) -> u64;
    fn free_id(&self) -> u64;
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

    fn gate(&mut self, gate: GateOwned) -> u64 {
        if gate.cacheable() {
            match self.cache.get(&gate) {
                Some(cached) => *cached,
                None => {
                    let id = self.builder.gate(gate.clone());
                    self.cache.insert(gate, id);
                    id
                }
            }
        } else {
            self.builder.gate(gate)
        }
    }
}
