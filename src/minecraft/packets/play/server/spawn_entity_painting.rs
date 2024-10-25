use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint, Position};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct SpawnEntityPainting {
    #[encoding("varint")]
    pub entity_id: i32,
    pub title: String,
    pub location: Position,
    pub direction: u8,
}
