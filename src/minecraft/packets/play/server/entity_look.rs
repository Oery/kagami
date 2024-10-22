use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityLook {
    #[encoding("varint")]
    entity_id: i32,
    yaw: i8,
    pitch: i8,
    on_ground: bool,
}
