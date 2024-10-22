use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityRelativeMove {
    #[encoding("varint")]
    entity_id: i32,
    d_x: i8,
    d_y: i8,
    d_z: i8,
    on_ground: bool,
}
