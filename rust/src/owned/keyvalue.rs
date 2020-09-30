use serde::{Deserialize, Serialize};
use std::iter::FromIterator;

use flatbuffers::{FlatBufferBuilder, WIPOffset, Vector, ForwardsUOffset};
use crate::zkinterface_generated::zkinterface as fb;

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct KeyValueOwned {
    pub key: String,
    // The value goes into one the following:
    pub text: Option<String>,
    pub data: Option<Vec<u8>>,
    pub number: i64,
}

impl<K: ToString> From<(K, String)> for KeyValueOwned {
    fn from((key, text): (K, String)) -> Self {
        Self { key: key.to_string(), text: Some(text), ..Self::default() }
    }
}

impl<K: ToString> From<(K, &str)> for KeyValueOwned {
    fn from((key, text): (K, &str)) -> Self {
        Self { key: key.to_string(), text: Some(text.to_string()), ..Self::default() }
    }
}

impl<K: ToString> From<(K, Vec<u8>)> for KeyValueOwned {
    fn from((key, data): (K, Vec<u8>)) -> Self {
        Self { key: key.to_string(), data: Some(data), ..Self::default() }
    }
}

impl<K: ToString> From<(K, i64)> for KeyValueOwned {
    fn from((key, number): (K, i64)) -> Self {
        Self { key: key.to_string(), number, ..Self::default() }
    }
}

impl<'a> From<fb::KeyValue<'a>> for KeyValueOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(kv_ref: fb::KeyValue) -> KeyValueOwned {
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
        vec_kv_ref: Option<Vector<ForwardsUOffset<fb::KeyValue>>>)
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
    ) -> WIPOffset<fb::KeyValue<'bldr>>
    {
        let key = Some(builder.create_string(&self.key));

        let text = self.text.as_ref().map(|text|
            builder.create_string(text));

        let data = self.data.as_ref().map(|data|
            builder.create_vector(data));

        fb::KeyValue::create(builder, &fb::KeyValueArgs {
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
    ) -> WIPOffset<Vector<'bldr, ForwardsUOffset<fb::KeyValue<'bldr>>>>
    {
        let vec_kv_ref = Vec::from_iter(kv_vec.iter().map(|kv|
            kv.build(builder)));

        builder.create_vector(&vec_kv_ref)
    }
}
