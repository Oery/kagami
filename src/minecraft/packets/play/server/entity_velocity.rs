use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityVelocity {
    #[encoding("varint")]
    entity_id: i32,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}
