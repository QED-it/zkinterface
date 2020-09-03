use crate::{GateOwned, GateSystemOwned};

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
    free_id: u64,
    pub gate_system: GateSystemOwned,
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