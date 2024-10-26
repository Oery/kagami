use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x15, server)]
pub struct EntityRelativeMove {
    #[encoding("varint")]
    pub entity_id: i32,
    pub d_x: i8,
    pub d_y: i8,
    pub d_z: i8,
    pub on_ground: bool,
}
