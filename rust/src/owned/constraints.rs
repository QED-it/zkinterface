use owned::variables::VariablesOwned;
use serde::{Deserialize, Serialize};
use zkinterface_generated::zkinterface::ConstraintSystem;
use std::io;
use flatbuffers::FlatBufferBuilder;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ConstraintSystemOwned {
    pub constraints: Vec<BilinearConstraintOwned>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct BilinearConstraintOwned {
    pub linear_combination_a: VariablesOwned,
    pub linear_combination_b: VariablesOwned,
    pub linear_combination_c: VariablesOwned,
}

impl<'a> From<ConstraintSystem<'a>> for ConstraintSystemOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(constraints_ref: ConstraintSystem) -> ConstraintSystemOwned {
        let mut owned = ConstraintSystemOwned {
            constraints: vec![],
        };

        let cons_ref = constraints_ref.constraints().unwrap();
        for i in 0..cons_ref.len() {
            let con_ref = cons_ref.get(i);
            owned.constraints.push(BilinearConstraintOwned {
                linear_combination_a: VariablesOwned::from(con_ref.linear_combination_a().unwrap()),
                linear_combination_b: VariablesOwned::from(con_ref.linear_combination_b().unwrap()),
                linear_combination_c: VariablesOwned::from(con_ref.linear_combination_c().unwrap()),
            });
        }

        owned
    }
}
// impl ConstraintSystemOwned {
//     pub fn write_constrains<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
//         let mut builder = &mut FlatBufferBuilder::new();
//         let mut constraints_built = vec![];
//
//         for (lca, lcb, lcc) in self {
//             let lca = VariablesOwned {
//                 variable_ids: lca.0,
//                 values: Some(lca.1),
//             }.build(builder);
//             let lcb = VariablesOwned {
//                 variable_ids: lcb.0,
//                 values: Some(lcb.1),
//             }.build(builder);
//             let lcc = VariablesOwned {
//                 variable_ids: lcc.0,
//                 values: Some(lcc.1),
//             }.build(builder);
//
//             constraints_built.push(BilinearConstraint::create(builder, &BilinearConstraintArgs {
//                 linear_combination_a: Some(lca),
//                 linear_combination_b: Some(lcb),
//                 linear_combination_c: Some(lcc),
//             }));
//         }
//
//         let constraints_built = builder.create_vector(&constraints_built);
//         let r1cs = ConstraintSystem::create(&mut builder, &ConstraintSystemArgs {
//             constraints: Some(constraints_built),
//             info: None,
//         });
//
//         let message = Root::create(&mut builder, &RootArgs {
//             message_type: Message::ConstraintSystem,
//             message: Some(r1cs.as_union_value()),
//         });
//         builder.finish_size_prefixed(message, None);
//
//         writer.write_all(builder.finished_data())
//     }
// }