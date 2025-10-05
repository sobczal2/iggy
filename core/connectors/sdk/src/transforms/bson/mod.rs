use bson::Bson;
use chrono::Utc;

use crate::transforms::ComputedValue;

pub mod add_fields;

/// Computes a BSON value based on the specified computed value type
pub fn compute_value(kind: &ComputedValue) -> Bson {
    let now = Utc::now();
    match kind {
        ComputedValue::DateTime => now.to_rfc3339().into(),
        ComputedValue::TimestampNanos => now.timestamp_nanos_opt().unwrap().into(),
        ComputedValue::TimestampMicros => now.timestamp_micros().into(),
        ComputedValue::TimestampMillis => now.timestamp_millis().into(),
        ComputedValue::TimestampSeconds => now.timestamp().into(),
        ComputedValue::UuidV4 => uuid::Uuid::new_v4().to_string().into(),
        ComputedValue::UuidV7 => uuid::Uuid::now_v7().to_string().into(),
    }
}

#[cfg(test)]
pub mod test_utils;
