//! Helpers to read messages.

use flatbuffers::{read_scalar_at, SIZE_UOFFSET, UOffsetT};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::zkinterface_generated::zkinterface as fb;
use crate::Result;

pub fn read_circuit_header(msg: &[u8]) -> Result<fb::CircuitHeader> {
    fb::get_size_prefixed_root_as_root(msg)
        .message_as_circuit_header().ok_or("Not a CircuitHeader message".into())
}

pub fn parse_header(msg: &[u8]) -> Option<(fb::CircuitHeader, Vec<Variable>)> {
    let header = fb::get_size_prefixed_root_as_root(msg).message_as_circuit_header()?;
    let input_var_ids = header.instance_variables()?.variable_ids()?.safe_slice();

    let assigned = match header.instance_variables()?.values() {
        Some(bytes) => {
            let stride = get_value_size(input_var_ids, bytes);

            (0..input_var_ids.len()).map(|i|
                Variable {
                    id: input_var_ids[i],
                    value: &bytes[stride * i..stride * (i + 1)],
                }
            ).collect()
        }
        None => vec![],
    };

    Some((header, assigned))
}

pub fn is_contiguous(mut first_id: u64, ids: &[u64]) -> bool {
    for id in ids {
        if *id != first_id { return false; }
        first_id += 1;
    }
    true
}

// Read a flatbuffers size prefix (4 bytes, little-endian). Size including the prefix.
pub fn read_size_prefix(buf: &[u8]) -> usize {
    if buf.len() < SIZE_UOFFSET { return 0; }
    let size = read_scalar_at::<UOffsetT>(buf, 0) as usize;
    SIZE_UOFFSET + size
}

pub fn split_messages(mut buf: &[u8]) -> Vec<&[u8]> {
    let mut bufs = vec![];
    loop {
        let size = read_size_prefix(buf);
        if size <= SIZE_UOFFSET { break; }
        bufs.push(&buf[..size]);
        buf = &buf[size..];
    }
    bufs
}

pub fn read_buffer(mut stream: impl Read) -> Result<Vec<u8>> {
    let mut buffer = vec![0u8; 4];
    if stream.read_exact(&mut buffer).is_err() {
        return Ok(Vec::new()); // End of stream at the correct place.
    }
    let size = read_size_prefix(&buffer);
    //eprintln!("Read size: {:?} --> size={}", buffer, size);
    if size <= SIZE_UOFFSET {
        return Ok(Vec::new()); // Explicit size 0 as end marker.
    }
    buffer.resize(size, 0);
    stream.read_exact(&mut buffer[4..])?;
    //eprintln!("Read buffer: {:?}", buffer);
    Ok(buffer)
}

pub fn get_value_size(var_ids: &[u64], values: &[u8]) -> usize {
    if var_ids.len() == 0 { 0 } else { values.len() / var_ids.len() }
}

/// Collect buffers waiting to be read.
#[derive(Clone)]
pub struct Reader {
    pub messages: Vec<Vec<u8>>,
    pub first_id: u64,
}

impl fmt::Debug for Reader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use fb::Message::*;

        let mut has_header = false;
        let mut has_witness = false;
        let mut has_constraints = false;

        for root in self.into_iter() {
            match root.message_type() {
                CircuitHeader => has_header = true,
                Witness => has_witness = true,
                ConstraintSystem => has_constraints = true,
                Command => {}
                NONE => {}
            }
        }

        if has_header {
            write!(f, "\nZkInterface {:?}\n", CircuitHeader)?;
            if let Some(vars) = self.instance_variables() {
                write!(f, "Public variables:\n")?;
                for var in vars {
                    write!(f, "- {:?}\n", var)?;
                }
            }
            if let Some(header) = self.last_header() {
                write!(f, "Free variable id: {}\n", header.free_variable_id())?;
            }
        }

        if has_witness {
            write!(f, "\nZkInterface {:?}\n", Witness)?;
            if let Some(vars) = self.private_variables() {
                write!(f, "Private variables:\n")?;
                for var in vars {
                    write!(f, "- {:?}\n", var)?;
                }
            }
        }

        if has_constraints {
            write!(f, "\nZkInterface {:?}\n", ConstraintSystem)?;
            for constraint in self.iter_constraints() {
                write!(f, "{:?}\n", constraint)?;
            }
        }

        Ok(())
    }
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            messages: vec![],
            first_id: 1,
        }
    }

    /// Collect messages. Methods will filter out irrelevant variables.
    /// first_id: The first variable ID to consider in received messages.
    /// Variables with lower IDs are ignored.
    pub fn new_filtered(first_id: u64) -> Reader {
        Reader {
            messages: vec![],
            first_id,
        }
    }

    pub fn push_message(&mut self, buf: Vec<u8>) -> Result<()> {
        self.messages.push(buf);
        Ok(())
    }

    pub fn read_from(&mut self, mut reader: impl Read) -> Result<()> {
        loop {
            let buffer = read_buffer(&mut reader)?;
            if buffer.len() == 0 {
                return Ok(());
            }
            self.push_message(buffer)?;
        }
    }

    pub fn read_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let mut file = File::open(&path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        self.push_message(buf)
    }

    pub fn first_header(&self) -> Option<fb::CircuitHeader> {
        for message in self {
            match message.message_as_circuit_header() {
                Some(ret) => return Some(ret),
                None => continue,
            };
        }
        None
    }

    pub fn last_header(&self) -> Option<fb::CircuitHeader> {
        let returns = self.headers();
        if returns.len() > 0 {
            Some(returns[returns.len() - 1])
        } else { None }
    }

    pub fn headers(&self) -> Vec<fb::CircuitHeader> {
        let mut returns = vec![];
        for message in self {
            match message.message_as_circuit_header() {
                Some(ret) => returns.push(ret),
                None => continue,
            };
        }
        returns
    }

    pub fn instance_variables(&self) -> Option<Vec<Variable>> {
        let instance_variables = self.last_header()?.instance_variables()?;
        collect_instance_variables(&instance_variables, self.first_id)
    }

    pub fn private_variables(&self) -> Option<Vec<Variable>> {
        // Collect private variables.
        let header = self.last_header()?;
        let mut vars = collect_unassigned_private_variables(
            &header.instance_variables()?,
            self.first_id,
            header.free_variable_id())?;

        // Collect assigned values, if any.
        let mut values = HashMap::with_capacity(vars.len());

        for assigned_var in self.iter_witness() {
            values.insert(assigned_var.id, assigned_var.value);
        }

        // Assign the values, if any.
        if values.len() > 0 {
            for var in vars.iter_mut() {
                if let Some(value) = values.get(&var.id) {
                    var.value = value;
                }
            }
        }

        Some(vars)
    }
}

pub fn collect_instance_variables<'a>(conn: &fb::Variables<'a>, first_id: u64) -> Option<Vec<Variable<'a>>> {
    let var_ids = conn.variable_ids()?.safe_slice();

    let values = match conn.values() {
        Some(values) => values,
        None => &[], // No values, only variable ids and empty values.
    };

    let stride = get_value_size(var_ids, values);

    let vars = (0..var_ids.len())
        .filter(|&i| // Ignore variables below first_id, if any.
            var_ids[i] >= first_id
        ).map(|i|          // Extract value of each variable.
        Variable {
            id: var_ids[i],
            value: &values[stride * i..stride * (i + 1)],
        }
    ).collect();

    Some(vars)
}

pub fn collect_unassigned_private_variables<'a>(instance_variables: &fb::Variables<'a>, first_id: u64, free_id: u64) -> Option<Vec<Variable<'a>>> {
    let var_ids = instance_variables.variable_ids()?.safe_slice();

    let vars = (first_id..free_id)
        .filter(|id| // Ignore variables already in the instance_variables.
            !var_ids.contains(id)
        ).map(|id|    // Variable without value.
        Variable {
            id,
            value: &[],
        }
    ).collect();

    Some(vars)
}

// Implement `for message in messages`
impl<'a> IntoIterator for &'a Reader {
    type Item = fb::Root<'a>;
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
    type Item = fb::Root<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.bufs.len() == 0 { return None; }

            let buf = &self.bufs[0][self.offset..];

            let size = {
                let size = read_size_prefix(buf);
                if size <= buf.len() {
                    size
                } else {
                    buf.len()
                }
            };

            if size <= SIZE_UOFFSET {
                // Move to the next buffer.
                self.bufs = &self.bufs[1..];
                self.offset = 0;
                continue;
            }

            // Move to the next message in the current buffer.
            self.offset += size;

            // Parse the current message.
            let root = fb::get_size_prefixed_root_as_root(&buf[..size]);
            return Some(root);
        }
    }
}


// R1CS messages
impl Reader {
    pub fn iter_constraints(&self) -> R1CSIterator {
        R1CSIterator {
            messages_iter: self.into_iter(),
            constraints_count: 0,
            next_constraint: 0,
            constraints: None,
        }
    }
}

pub type Term<'a> = Variable<'a>;

#[derive(Debug, Eq, PartialEq)]
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
    constraints: Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<fb::BilinearConstraint<'a>>>>,
}

impl<'a> Iterator for R1CSIterator<'a> {
    type Item = Constraint<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_constraint >= self.constraints_count {
            // Grab the next message, or terminate if none.
            let message = self.messages_iter.next()?;

            // Parse the message, skip irrelevant message types, or fail if invalid.
            let constraints = match message.message_as_constraint_system() {
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

        fn to_vec<'a>(lc: fb::Variables<'a>) -> Vec<Term<'a>> {
            let mut terms = vec![];
            let var_ids: &[u64] = lc.variable_ids().unwrap().safe_slice();
            let values: &[u8] = lc.values().unwrap();

            let stride = get_value_size(var_ids, values);

            for i in 0..var_ids.len() {
                terms.push(Term {
                    id: var_ids[i],
                    value: &values[stride * i..stride * (i + 1)],
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
impl Reader {
    pub fn iter_witness(&self) -> WitnessIterator {
        WitnessIterator {
            messages_iter: self.into_iter(),
            var_ids: &[],
            values: &[],
            next_element: 0,
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct Variable<'a> {
    pub id: u64,
    pub value: &'a [u8],
}

impl<'a> Variable<'a> {
    pub fn has_value(&self) -> bool {
        self.value.len() > 0
    }

    pub fn is_constant_one(&self) -> bool {
        if self.id != 0 {
            return false;
        }
        if self.value.len() > 0 {
            if self.value[0] != 1 {
                return false;
            }
            for v in self.value[1..].iter() {
                if *v != 0 {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<'a> fmt::Debug for Variable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.value.len();
        if len == 0 {
            write!(f, "var_{}", self.id)
        } else {
            write!(f, "var_{}=[{:?}", self.id, self.value[0])?;

            // Find length before trailing zeros.
            let mut trail = 1;
            for i in (0..len).rev() {
                if self.value[i] != 0 {
                    trail = i + 1;
                    break;
                }
            }

            for b in self.value[1..trail].iter() {
                write!(f, ",{}", b)?;
            }
            write!(f, "]")
        }
    }
}

pub struct WitnessIterator<'a> {
    // Iterate over messages.
    messages_iter: MessageIterator<'a>,

    // Iterate over variables in the current message.
    var_ids: &'a [u64],
    values: &'a [u8],
    next_element: usize,
}

impl<'a> Iterator for WitnessIterator<'a> {
    type Item = Variable<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_element >= self.var_ids.len() {
            // Grab the next message, or terminate if none.
            let message = self.messages_iter.next()?;

            // Parse the message, skip irrelevant message types, or fail if invalid.
            let witness = match message.message_as_witness() {
                Some(message) => message.assigned_variables().unwrap(),
                None => continue,
            };

            // Start iterating the values of the current message.
            self.var_ids = witness.variable_ids().unwrap().safe_slice();
            self.values = witness.values().unwrap();
            self.next_element = 0;
        }

        let stride = get_value_size(self.var_ids, self.values);

        let i = self.next_element;
        self.next_element += 1;

        Some(Variable {
            id: self.var_ids[i],
            value: &self.values[stride * i..stride * (i + 1)],
        })
    }
    // TODO: Replace unwrap and panic with Result.
}

#[test]
fn test_pretty_print_var() {
    assert_eq!(format!("{:?}", Variable {
        id: 1,
        value: &[],
    }), "var_1");
    assert_eq!(format!("{:?}", Variable {
        id: 2,
        value: &[9],
    }), "var_2=[9]");
    assert_eq!(format!("{:?}", Variable {
        id: 2,
        value: &[9, 0],
    }), "var_2=[9]");
    assert_eq!(format!("{:?}", Variable {
        id: 2,
        value: &[9, 8],
    }), "var_2=[9,8]");
    assert_eq!(format!("{:?}", Variable {
        id: 3,
        value: &[9, 8, 7, 6],
    }), "var_3=[9,8,7,6]");
    assert_eq!(format!("{:?}", Variable {
        id: 3,
        value: &[9, 8, 0, 6],
    }), "var_3=[9,8,0,6]");
    assert_eq!(format!("{:?}", Variable {
        id: 4,
        value: &[9, 8, 0, 6, 0, 0],
    }), "var_4=[9,8,0,6]");
}
