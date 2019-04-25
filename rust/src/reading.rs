//! Helpers to read messages.

use flatbuffers::{read_scalar_at, SIZE_UOFFSET, UOffsetT};
use zkinterface_generated::zkinterface::{
    BilinearConstraint,
    GadgetCall,
    get_size_prefixed_root_as_root,
    VariableValues,
};
use zkinterface_generated::zkinterface::Root;

pub fn parse_call(call_msg: &[u8]) -> Option<(GadgetCall, Vec<AssignedVariable>)> {
    let call = get_size_prefixed_root_as_root(call_msg).message_as_gadget_call()?;
    let input_var_ids = call.connections()?.variable_ids()?.safe_slice();

    let assigned = if call.generate_assignment() {
        let bytes = call.connections()?.values()?;
        let stride = bytes.len() / input_var_ids.len();

        (0..input_var_ids.len()).map(|i|
            AssignedVariable {
                id: input_var_ids[i],
                element: &bytes[stride * i..stride * (i + 1)],
            }
        ).collect()
    } else {
        vec![]
    };

    Some((call, assigned))
}

pub fn is_contiguous(mut first_id: u64, ids: &[u64]) -> bool {
    for id in ids {
        if *id != first_id { return false; }
        first_id += 1;
    }
    true
}

pub fn read_size(buf: &[u8]) -> usize {
    if buf.len() < SIZE_UOFFSET { return 0; }
    let size = read_scalar_at::<UOffsetT>(buf, 0) as usize;
    SIZE_UOFFSET + size
}

pub fn split_messages(mut buf: &[u8]) -> Vec<&[u8]> {
    let mut bufs = vec![];
    loop {
        let size = read_size(buf);
        if size == 0 { break; }
        bufs.push(&buf[..size]);
        buf = &buf[size..];
    }
    bufs
}

/// Collect buffers waiting to be read.
#[derive(Clone, Debug)]
pub struct Messages {
    pub messages: Vec<Vec<u8>>,
}

impl Messages {
    pub fn new() -> Messages {
        Messages {
            messages: vec![],
        }
    }

    pub fn push_message(&mut self, buf: Vec<u8>) -> Result<(), String> {
        self.messages.push(buf);
        Ok(())
    }

    // Return message
    pub fn last_gadget_return(&self) -> Option<GadgetCall> {
        let returns = self.gadget_returns();
        if returns.len() > 0 {
            Some(returns[returns.len() - 1])
        } else { None }
    }

    pub fn gadget_returns(&self) -> Vec<GadgetCall> {
        let mut returns = vec![];
        for message in self {
            match message.message_as_gadget_call() {
                Some(ret) => returns.push(ret),
                None => continue,
            };
        }
        returns
    }
}

impl<'a> IntoIterator for &'a Messages {
    type Item = Root<'a>;
    type IntoIter = MessageIterator<'a>;

    fn into_iter(self) -> MessageIterator<'a> {
        MessageIterator {
            bufs: &self.messages,
            offset: 0,
        }
    }
}

pub struct MessageIterator<'a> {
    bufs: &'a [Vec<u8>],
    offset: usize,
}

impl<'a> Iterator for MessageIterator<'a> {
    type Item = Root<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.bufs.len() == 0 { return None; }

            let buf = &self.bufs[0][self.offset..];

            let size = {
                let size = read_size(buf);
                if size <= buf.len() {
                    size
                } else {
                    buf.len()
                }
            };

            if size == 0 {
                // Move to the next buffer.
                self.bufs = &self.bufs[1..];
                self.offset = 0;
                continue;
            }

            // Move to the next message in the current buffer.
            self.offset += size;

            // Parse the current message.
            let root = get_size_prefixed_root_as_root(&buf[..size]);
            return Some(root);
        }
    }
}


// R1CS messages
impl Messages {
    pub fn iter_constraints(&self) -> R1CSIterator {
        R1CSIterator {
            messages_iter: self.into_iter(),
            constraints_count: 0,
            next_constraint: 0,
            constraints: None,
        }
    }
}

pub type Term<'a> = AssignedVariable<'a>;

#[derive(Debug)]
pub struct Constraint<'a> {
    pub a: Vec<Term<'a>>,
    pub b: Vec<Term<'a>>,
    pub c: Vec<Term<'a>>,
}

pub struct R1CSIterator<'a> {
    // Iterate over messages.
    messages_iter: MessageIterator<'a>,

    // Iterate over constraints in the current message.
    constraints_count: usize,
    next_constraint: usize,
    constraints: Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BilinearConstraint<'a>>>>,
}

impl<'a> Iterator for R1CSIterator<'a> {
    type Item = Constraint<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_constraint >= self.constraints_count {
            // Grab the next message, or terminate if none.
            let message = self.messages_iter.next()?;

            // Parse the message, skip irrelevant message types, or fail if invalid.
            let constraints = match message.message_as_r1csconstraints() {
                Some(message) => message.constraints().unwrap(),
                None => continue,
            };

            // Start iterating the elements of the current message.
            self.constraints_count = constraints.len();
            self.next_constraint = 0;
            self.constraints = Some(constraints);
        }

        let constraint = self.constraints.as_ref().unwrap().get(self.next_constraint);
        self.next_constraint += 1;

        fn to_vec<'a>(lc: VariableValues<'a>) -> Vec<Term<'a>> {
            let mut terms = vec![];
            let var_ids: &[u64] = lc.variable_ids().unwrap().safe_slice();
            let values: &[u8] = lc.values().unwrap();

            let stride = values.len() / var_ids.len();

            for i in 0..var_ids.len() {
                terms.push(Term {
                    id: var_ids[i],
                    element: &values[stride * i..stride * (i + 1)],
                });
            }

            terms
        }

        Some(Constraint {
            a: to_vec(constraint.linear_combination_a().unwrap()),
            b: to_vec(constraint.linear_combination_b().unwrap()),
            c: to_vec(constraint.linear_combination_c().unwrap()),
        })
    }
    // TODO: Replace unwrap and panic with Result.
}


// Assignment messages
impl Messages {
    pub fn iter_assignment(&self) -> AssignedVariablesIterator {
        AssignedVariablesIterator {
            messages_iter: self.into_iter(),
            var_ids: &[],
            values: &[],
            next_element: 0,
        }
    }

    pub fn outgoing_assigned_variables(&self, first_id: u64) -> Option<Vec<AssignedVariable>> {
        let outputs = self.last_gadget_return()?.connections()?;
        let output_var_ids = outputs.variable_ids()?.safe_slice();
        let values = outputs.values()?;

        let stride = values.len() / output_var_ids.len();

        let mut assigned = vec![];
        for i in 0..output_var_ids.len() {
            if output_var_ids[i] >= first_id {
                assigned.push(AssignedVariable {
                    id: output_var_ids[i],
                    element: &values[stride * i..stride * (i + 1)],
                });
            }
        }

        Some(assigned)
    }
}

#[derive(Debug)]
pub struct AssignedVariable<'a> {
    pub id: u64,
    pub element: &'a [u8],
}

pub struct AssignedVariablesIterator<'a> {
    // Iterate over messages.
    messages_iter: MessageIterator<'a>,

    // Iterate over variables in the current message.
    var_ids: &'a [u64],
    values: &'a [u8],
    next_element: usize,
}

impl<'a> Iterator for AssignedVariablesIterator<'a> {
    type Item = AssignedVariable<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_element >= self.var_ids.len() {
            // Grab the next message, or terminate if none.
            let message = self.messages_iter.next()?;

            // Parse the message, skip irrelevant message types, or fail if invalid.
            let assigned_variables = match message.message_as_assigned_variables() {
                Some(message) => message.values().unwrap(),
                None => continue,
            };

            // Start iterating the values of the current message.
            self.var_ids = assigned_variables.variable_ids().unwrap().safe_slice();
            self.values = assigned_variables.values().unwrap();
            self.next_element = 0;
        }

        let stride = self.values.len() / self.var_ids.len();
        //if stride == 0 { panic!("Empty values data."); }

        let i = self.next_element;
        self.next_element += 1;

        Some(AssignedVariable {
            id: self.var_ids[i],
            element: &self.values[stride * i..stride * (i + 1)],
        })
    }
    // TODO: Replace unwrap and panic with Result.
}
