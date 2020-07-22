use std::io;

use flatbuffers::FlatBufferBuilder;
use serde::{Deserialize, Serialize};
use owned::variables::VariablesOwned;
use zkinterface_generated::zkinterface::{BilinearConstraint, BilinearConstraintArgs, ConstraintSystem, ConstraintSystemArgs, Message, Root, RootArgs};

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

impl From<&[((Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>))]> for ConstraintSystemOwned {
    /// Creates a `ConstraintSystemOwned` from an R1CS vector
    ///
    /// # Examples
    /// ```
    ///  constraints_vec: &[((Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>))] = &[
    ///         // (A ids values)  *  (B ids values)  =  (C ids values)
    ///         ((vec![1], vec![1]), (vec![1], vec![1]), (vec![4], vec![1])),       // x * x = xx
    ///         ((vec![2], vec![1]), (vec![2], vec![1]), (vec![5], vec![1])),       // y * y = yy
    ///         ((vec![0], vec![1]), (vec![4, 5], vec![1, 1]), (vec![3], vec![1])), // 1 * (xx + yy) = z
    ///  ];
    ///
    ///  let constraints = ConstraintSystemOwned::from(vec);
    ///```

    fn from(constraints: &[((Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>))]) -> ConstraintSystemOwned {
        let mut constraints_owned = ConstraintSystemOwned {
            constraints: vec![]
        };

        for (lca, lcb, lcc) in constraints {
            let lca = VariablesOwned {
                variable_ids: lca.0.clone(),
                values: Some(lca.1.clone()),
            };
            let lcb = VariablesOwned {
                variable_ids: lcb.0.clone(),
                values: Some(lcb.1.clone()),
            };
            let lcc = VariablesOwned {
                variable_ids: lcc.0.clone(),
                values: Some(lcc.1.clone()),
            };
            constraints_owned.constraints.push(BilinearConstraintOwned {
                linear_combination_a: lca,
                linear_combination_b: lcb,
                linear_combination_c: lcc,
            });
        }
        constraints_owned
    }
}

impl ConstraintSystemOwned {
    /// Writes the constraint system into the provided buffer
    ///
    ///
    ///  # Examples
    /// ```
    ///  let mut buf = Vec::<u8>::new();
    ///  constraints_owned.write(&mut buf).unwrap();
    /// ```

    pub fn write<W: io::Write>(self, writer: &mut W) -> io::Result<()> {
        let mut builder = &mut FlatBufferBuilder::new();
        let mut constraints_built = vec![];

        for bilinear_constraints in self.constraints {
            let lca = bilinear_constraints.linear_combination_a.build(builder);
            let lcb = bilinear_constraints.linear_combination_b.build(builder);
            let lcc = bilinear_constraints.linear_combination_c.build(builder);

            constraints_built.push(BilinearConstraint::create(builder, &BilinearConstraintArgs {
                linear_combination_a: Some(lca),
                linear_combination_b: Some(lcb),
                linear_combination_c: Some(lcc),
            }));
        }

        let constraints_built = builder.create_vector(&constraints_built);
        let r1cs = ConstraintSystem::create(&mut builder, &ConstraintSystemArgs {
            constraints: Some(constraints_built),
            info: None,
        });

        let message = Root::create(&mut builder, &RootArgs {
            message_type: Message::ConstraintSystem,
            message: Some(r1cs.as_union_value()),
        });
        builder.finish_size_prefixed(message, None);

        writer.write_all(builder.finished_data())
    }
}

