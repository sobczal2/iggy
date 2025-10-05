use bson::{Bson, Document};
use simd_json::{OwnedValue, StaticNode};

use crate::{transforms::{bson::compute_value, AddFields, FieldValue}, DecodedMessage, Error, Payload, TopicMetadata};

impl AddFields {
    pub(crate) fn transform_bson(
        &self,
        _metadata: &TopicMetadata,
        mut message: DecodedMessage,
    ) -> Result<Option<DecodedMessage>, Error> {
        let Payload::Bson(Bson::Document(ref mut document)) = message.payload else {
            return Ok(Some(message));
        };

        for field in &self.fields {
            let new_val = match &field.value {
                FieldValue::Static(v) => owned_value_to_bson(v.clone()),
                FieldValue::Computed(c) => compute_value(c),
            };
            document.insert(field.key.clone(), new_val);
        }

        Ok(Some(message))
    }
}

fn owned_value_to_bson(value: OwnedValue) -> Bson {
    match value {
        OwnedValue::Static(StaticNode::Null) => Bson::Null,
        OwnedValue::Static(StaticNode::I64(v)) => {
            if (i32::MIN as i64..=i32::MAX as i64).contains(&v) {
                Bson::Int32(v as i32)
            } else {
                Bson::Int64(v)
            }
        },
        OwnedValue::Static(StaticNode::U64(v)) => {
            if v <= i32::MAX as u64
            {
                Bson::Int32(v as i32)
            } else if v <= i64::MAX as u64 {
                Bson::Int64(v as i64)
            } else {
                Bson::Double(v as f64)
            }
        },
        OwnedValue::Static(StaticNode::F64(v)) => Bson::Double(v),
        OwnedValue::Static(StaticNode::Bool(v)) => Bson::Boolean(v),
        OwnedValue::String(v) => Bson::String(v.clone()),
        OwnedValue::Array(arr) => Bson::Array(arr.into_iter().map(owned_value_to_bson).collect()),
        OwnedValue::Object(obj) => {
            let mut doc = Document::new();
            for (k, v) in obj.into_iter() {
                doc.insert(k, owned_value_to_bson(v));
            }

            Bson::Document(doc)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bson::doc;
    use simd_json::derived::MutableObject;
    use simd_json::{OwnedValue, ValueBuilder};

    #[test]
    fn owned_value_to_bson_should_correctly_convert_each_data_type() {
        let mut json = OwnedValue::object();

        json.insert("f_null", OwnedValue::Static(StaticNode::Null)).unwrap();
        json.insert("f_i64_i32", OwnedValue::Static(StaticNode::I64(-10))).unwrap();
        json.insert("f_i64_i64", OwnedValue::Static(StaticNode::I64(3_000_000_000))).unwrap();
        json.insert("f_u64_i32", OwnedValue::Static(StaticNode::U64(10))).unwrap();
        json.insert("f_u64_i64", OwnedValue::Static(StaticNode::U64(3_000_000_000))).unwrap();
        json.insert("f_u64_f64", OwnedValue::Static(StaticNode::U64(10_000_000_000_000_000_000))).unwrap();
        json.insert("f_f64", OwnedValue::Static(StaticNode::F64(1.123))).unwrap();
        json.insert("f_bool", OwnedValue::Static(StaticNode::Bool(true))).unwrap();
        json.insert("f_string", OwnedValue::String("test".into())).unwrap();

        json.insert("f_array", OwnedValue::Array(Box::new(vec![
            OwnedValue::Static(StaticNode::I64(1)),
            OwnedValue::String("two".into())
        ]))).unwrap();

        let mut nested = OwnedValue::object();
        nested.insert("nested_key", OwnedValue::Static(StaticNode::Bool(false))).unwrap();
        json.insert("f_object", nested).unwrap();

        let bson = owned_value_to_bson(json);

        let expected = bson::Bson::Document(doc! {
            "f_null": null,
            "f_i64_i32": -10,
            "f_i64_i64": 3_000_000_000i64,
            "f_u64_i32": 10,
            "f_u64_i64": 3_000_000_000i64,
            "f_u64_f64": 10_000_000_000_000_000_000f64,
            "f_f64": 1.123,
            "f_bool": true,
            "f_string": "test",
            "f_array": [1, "two"],
            "f_object": { "nested_key": false },
        });

        assert_eq!(bson, expected);
    }
}
