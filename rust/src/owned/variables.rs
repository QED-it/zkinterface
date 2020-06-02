use serde::{Deserialize, Serialize};

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use zkinterface_generated::zkinterface::{
    Variables,
    VariablesArgs,
};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct VariablesOwned {
    pub variable_ids: Vec<u64>,
    pub values: Option<Vec<u8>>,
    // pub info: Option<Vec<(String, &'a [u8])>>,
}

impl<'a> From<Variables<'a>> for VariablesOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(variables_ref: Variables) -> VariablesOwned {
        VariablesOwned {
            variable_ids: match variables_ref.variable_ids() {
                Some(var_ids) => Vec::from(var_ids.safe_slice()),
                None => vec![],
            },
            values: variables_ref.values().map(|bytes|
                Vec::from(bytes)),
        }
    }
}

impl VariablesOwned {
    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Variables<'bldr>>
    {
        let variable_ids = Some(builder.create_vector(&self.variable_ids));

        let values = self.values.as_ref().map(|values|
            builder.create_vector(values));

        Variables::create(builder, &VariablesArgs {
            variable_ids,
            values,
            info: None,
        })
    }
}
