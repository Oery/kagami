use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityMetadata {
    #[encoding("varint")]
    pub entity_id: i32,
    pub metadata: Vec<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {}
