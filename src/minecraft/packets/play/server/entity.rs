use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x14)]
pub struct Entity {
    #[encoding("varint")]
    pub entity_id: i32,
}
