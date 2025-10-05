use crate::{DecodedMessage, Payload, TopicMetadata};
use bson::{Bson, Document};
use uuid;

/// Helper function to create a test message with the given BSON payload
pub fn create_test_message(bson: Bson) -> DecodedMessage {
    DecodedMessage {
        id: None,
        offset: None,
        checksum: None,
        timestamp: None,
        origin_timestamp: None,
        headers: None,
        payload: Payload::Bson(bson),
    }
}

/// Helper function to create a non-BSON message with raw bytes
pub fn create_raw_test_message(bytes: Vec<u8>) -> DecodedMessage {
    DecodedMessage {
        id: None,
        offset: None,
        checksum: None,
        timestamp: None,
        origin_timestamp: None,
        headers: None,
        payload: Payload::Raw(bytes),
    }
}

/// Helper function to create a topic metadata for testing
pub fn create_test_topic_metadata() -> TopicMetadata {
    TopicMetadata {
        stream: "test-stream".to_string(),
        topic: "test-topic".to_string(),
    }
}

/// Helper function to extract the BSON document from a message
pub fn extract_json_document(msg: &DecodedMessage) -> Option<&Document> {
    if let Payload::Bson(Bson::Document(document)) = &msg.payload {
        Some(document)
    } else {
        None
    }
}
