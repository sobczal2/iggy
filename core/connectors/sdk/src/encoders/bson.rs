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

use crate::{Error, Payload, Schema, StreamEncoder};

pub struct BsonStreamEncoder;

impl StreamEncoder for BsonStreamEncoder {
    fn schema(&self) -> Schema {
        Schema::Bson
    }

    fn encode(&self, payload: Payload) -> Result<Vec<u8>, Error> {
        match payload {
            Payload::Bson(value) => {
                bson::serialize_to_vec(&value).map_err(|_| Error::InvalidBsonPayload)
            }
            Payload::Json(value) => {
                bson::serialize_to_vec(&value).map_err(|_| Error::InvalidBsonPayload)
            }
            _ => Err(Error::InvalidPayloadType),
        }
    }
}

#[cfg(test)]
mod tests {
    use bson::bson;
    use simd_json::json;

    use super::*;

    #[test]
    fn encode_should_encode_bson_value_successfully() {
        let encoder = BsonStreamEncoder;

        let payload = Payload::Bson(bson!({
            "data": "test"
        }));

        let result = encoder.encode(payload);
        assert!(
            result.is_ok(),
            "Should encode bson data"
        );
    }

    #[test]
    fn encode_should_encode_json_value_successfully() {
        let encoder = BsonStreamEncoder;

        let payload = Payload::Json(json!({
            "data": "test"
        }));

        let result = encoder.encode(payload);
        assert!(
            result.is_ok(),
            "Should encode json data"
        );
    }
}
