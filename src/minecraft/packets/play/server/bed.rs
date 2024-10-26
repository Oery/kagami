use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint, Position};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x0A)]
pub struct Bed {
    #[encoding("varint")]
    pub entity_id: i32,
    pub location: Position,
}
