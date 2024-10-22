use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityHeadRotation {
    #[encoding("varint")]
    entity_id: i32,
    head_yaw: i8,
}
