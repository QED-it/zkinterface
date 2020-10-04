use serde::{Deserialize, Serialize};

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use crate::zkinterface_generated::zkinterface as fb;
use crate::consumers::reader::{Variable, get_value_size};

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Variables {
    pub variable_ids: Vec<u64>,
    pub values: Option<Vec<u8>>,
    // pub info: Option<Vec<(String, &'a [u8])>>,
}

impl<'a> From<fb::Variables<'a>> for Variables {
    /// Convert from Flatbuffers references to owned structure.
    fn from(fb_variables: fb::Variables) -> Variables {
        Variables {
            variable_ids: match fb_variables.variable_ids() {
                Some(var_ids) => Vec::from(var_ids.safe_slice()),
                None => vec![],
            },
            values: match fb_variables.values() {
                Some(bytes) => Some(Vec::from(bytes)),
                None => None,
            },
        }
    }
}

impl Variables {
    pub fn get_variables(&self) -> Vec<Variable> {
        let values = match self.values {
            Some(ref values) => values as &[u8],
            None => &[], // No values, only variable ids and empty values.
        };

        let stride = self.value_size();

        (0..self.variable_ids.len())
            .map(|var_i|
                Variable {
                    id: self.variable_ids[var_i],
                    // Extract the value. Possibly empty.
                    value: &values[stride * var_i..stride * (var_i + 1)],
                }
            ).collect()
    }

    pub fn value_size(&self) -> usize {
        match self.values {
            Some(ref values) => get_value_size(&self.variable_ids, values),
            None => 0,
        }
    }

    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<fb::Variables<'bldr>>
    {
        let variable_ids = Some(builder.create_vector(&self.variable_ids));

        let values = self.values.as_ref().map(|values|
            builder.create_vector(values));

        fb::Variables::create(builder, &fb::VariablesArgs {
            variable_ids,
            values,
            info: None,
        })
    }
}
