use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint, Position};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct SpawnEntityPainting {
    #[encoding("varint")]
    entity_id: i32,
    title: String,
    location: Position,
    direction: u8,
}
