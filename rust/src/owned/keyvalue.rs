use serde::{Deserialize, Serialize};
use std::iter::FromIterator;

use flatbuffers::{FlatBufferBuilder, WIPOffset, Vector, ForwardsUOffset};
use crate::zkinterface_generated::zkinterface::{
    KeyValue,
    KeyValueArgs,
};

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct KeyValueOwned {
    pub key: String,
    // The value goes into one the following:
    pub text: Option<String>,
    pub data: Option<Vec<u8>>,
    pub number: i64,
}

impl<'a> From<KeyValue<'a>> for KeyValueOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(kv_ref: KeyValue) -> KeyValueOwned {
        KeyValueOwned {
            key: kv_ref.key().unwrap().into(),
            text: kv_ref.text().map(|d| String::from(d)),
            data: kv_ref.data().map(|d| Vec::from(d)),
            number: kv_ref.number(),
        }
    }
}

impl KeyValueOwned {
    pub fn from_vector(
        vec_kv_ref: Option<Vector<ForwardsUOffset<KeyValue>>>)
        -> Option<Vec<KeyValueOwned>> {
        vec_kv_ref.map(|vec_kv_ref| {
            let mut vec_kv = vec![];
            for i in 0..vec_kv_ref.len() {
                let kv = vec_kv_ref.get(i);
                vec_kv.push(KeyValueOwned::from(kv));
            }
            vec_kv
        })
    }

    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<KeyValue<'bldr>>
    {
        let key = Some(builder.create_string(&self.key));

        let text = self.text.as_ref().map(|text|
            builder.create_string(text));

        let data = self.data.as_ref().map(|data|
            builder.create_vector(data));

        KeyValue::create(builder, &KeyValueArgs {
            key,
            text,
            data,
            number: self.number,
        })
    }

    /// Add a vector of these structures into a Flatbuffers message builder.
    pub fn build_vector<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        kv_vec: &'args [KeyValueOwned],
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Vector<'bldr, ForwardsUOffset<KeyValue<'bldr>>>>
    {
        let vec_kv_ref = Vec::from_iter(kv_vec.iter().map(|kv|
            kv.build(builder)));

        builder.create_vector(&vec_kv_ref)
    }
}
