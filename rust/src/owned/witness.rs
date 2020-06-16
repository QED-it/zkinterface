use owned::variables::VariablesOwned;
use serde::{Deserialize, Serialize};
use zkinterface_generated::zkinterface::Witness;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct WitnessOwned {
    assigned_variables: VariablesOwned,
}

impl<'a> From<Witness<'a>> for WitnessOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(witness_ref: Witness) -> WitnessOwned {
        WitnessOwned {
            assigned_variables: VariablesOwned::from(witness_ref.assigned_variables().unwrap()),
        }
    }
}
