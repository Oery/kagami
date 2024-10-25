use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityLook {
    #[encoding("varint")]
    pub entity_id: i32,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
