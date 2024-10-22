use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint, Position};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct Bed {
    #[encoding("varint")]
    pub entity_id: i32,
    pub location: Position,
}
