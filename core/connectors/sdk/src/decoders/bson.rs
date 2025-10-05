/* Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

use bson::Bson;

use crate::{Error, Payload, Schema, StreamDecoder};

pub struct BsonStreamDecoder;

impl StreamDecoder for BsonStreamDecoder {
    fn schema(&self) -> Schema {
        Schema::Bson
    }

    fn decode(&self, payload: Vec<u8>) -> Result<Payload, Error> {
        let bson: Bson = bson::deserialize_from_slice(&payload).map_err(|_| Error::InvalidBsonPayload)?;
        Ok(Payload::Bson(bson))
    }
}

#[cfg(test)]
mod tests {
    use bson::{bson, Bson};
    use simd_json::json;

    use crate::{encoders::bson::BsonStreamEncoder, StreamEncoder};

    use super::*;

    #[test]
    fn decode_should_decode_bson_value_successfully() {
        let encoder = BsonStreamEncoder;
        let decoder = BsonStreamDecoder;

        let payload = Payload::Bson(bson!({
            "data": "test"
        }));

        let encoded = encoder.encode(payload).expect("failed to encode payload");

        let result = decoder.decode(encoded);

        assert!(
            result.is_ok(),
            "Should decode bson data"
        );
        
        let document = if let Payload::Bson(document) = result.unwrap() {
            document
        } else {
            panic!("decoded not a bson payload");
        };

        assert_eq!(document.as_document().expect("not a document").get("data"), Some(&Bson::String("test".to_string())));
    }

    #[test]
    fn decode_should_decode_json_value_successfully() {
        let encoder = BsonStreamEncoder;
        let decoder = BsonStreamDecoder;

        let payload = Payload::Json(json!({
            "data": "test"
        }));

        let encoded = encoder.encode(payload).expect("failed to encode payload");

        let result = decoder.decode(encoded);

        assert!(
            result.is_ok(),
            "Should decode json data"
        );
        
        let document = if let Payload::Bson(document) = result.unwrap() {
            document
        } else {
            panic!("decoded not a bson payload");
        };

        assert_eq!(document.as_document().expect("not a document").get("data"), Some(&Bson::String("test".to_string())));
    }
}
