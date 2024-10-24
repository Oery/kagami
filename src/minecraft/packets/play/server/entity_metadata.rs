use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityMetadata {
    #[encoding("varint")]
    entity_id: i32,
    metadata: Vec<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Metadata {}
