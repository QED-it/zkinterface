use serde::{Deserialize, Serialize};
use std::iter::FromIterator;

use flatbuffers::{FlatBufferBuilder, WIPOffset, Vector, ForwardsUOffset};
use crate::zkinterface_generated::zkinterface as fb;

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct KeyValue {
    pub key: String,
    // The value goes into one the following:
    pub text: Option<String>,
    pub data: Option<Vec<u8>>,
    pub number: i64,
}

impl<K: ToString> From<(K, String)> for KeyValue {
    fn from((key, text): (K, String)) -> Self {
        Self { key: key.to_string(), text: Some(text), ..Self::default() }
    }
}

impl<K: ToString> From<(K, &str)> for KeyValue {
    fn from((key, text): (K, &str)) -> Self {
        Self { key: key.to_string(), text: Some(text.to_string()), ..Self::default() }
    }
}

impl<K: ToString> From<(K, Vec<u8>)> for KeyValue {
    fn from((key, data): (K, Vec<u8>)) -> Self {
        Self { key: key.to_string(), data: Some(data), ..Self::default() }
    }
}

impl<K: ToString> From<(K, i64)> for KeyValue {
    fn from((key, number): (K, i64)) -> Self {
        Self { key: key.to_string(), number, ..Self::default() }
    }
}

impl<'a> From<fb::KeyValue<'a>> for KeyValue {
    /// Convert from Flatbuffers references to owned structure.
    fn from(fb_kv: fb::KeyValue) -> KeyValue {
        KeyValue {
            key: fb_kv.key().unwrap().into(),
            text: fb_kv.text().map(|d| String::from(d)),
            data: fb_kv.data().map(|d| Vec::from(d)),
            number: fb_kv.number(),
        }
    }
}

impl KeyValue {
    pub fn from_vector(
        fb_key_values: Option<Vector<ForwardsUOffset<fb::KeyValue>>>
    ) -> Option<Vec<KeyValue>> {
        fb_key_values.map(|fb_key_values| {
            let mut key_values = vec![];
            for i in 0..fb_key_values.len() {
                let fb_key_value = fb_key_values.get(i);
                key_values.push(KeyValue::from(fb_key_value));
            }
            key_values
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
        key_values: &'args [KeyValue],
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Vector<'bldr, ForwardsUOffset<fb::KeyValue<'bldr>>>>
    {
        let fb_key_values = Vec::from_iter(key_values.iter().map(|kv|
            kv.build(builder)));

        builder.create_vector(&fb_key_values)
    }
}
