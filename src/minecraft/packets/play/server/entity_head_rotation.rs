use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityHeadRotation {
    #[encoding("varint")]
    pub entity_id: i32,
    pub head_yaw: i8,
}
