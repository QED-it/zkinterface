use owned::variables::VariablesOwned;
use serde::{Deserialize, Serialize};
use zkinterface_generated::zkinterface::R1CSConstraints;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ConstraintsOwned {
    constraints: Vec<ConstraintOwned>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ConstraintOwned {
    pub a: VariablesOwned,
    pub b: VariablesOwned,
    pub c: VariablesOwned,
}

impl<'a> From<R1CSConstraints<'a>> for ConstraintsOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(constraints_ref: R1CSConstraints) -> ConstraintsOwned {
        let mut owned = ConstraintsOwned {
            constraints: vec![],
        };

        let cons_ref = constraints_ref.constraints().unwrap();
        for i in 0..cons_ref.len() {
            let con_ref = cons_ref.get(i);
            owned.constraints.push(ConstraintOwned {
                a: VariablesOwned::from(con_ref.linear_combination_a().unwrap()),
                b: VariablesOwned::from(con_ref.linear_combination_b().unwrap()),
                c: VariablesOwned::from(con_ref.linear_combination_c().unwrap()),
            });
        }

        owned
    }
}
