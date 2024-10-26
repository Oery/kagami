use crate::minecraft::Packet;
use crate::serialization::{
    deserialize_fixed_point, deserialize_varint, serialize_fixed_point, serialize_varint,
};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x11, server)]
pub struct SpawnEntityExperienceOrb {
    #[encoding("varint")]
    pub entity_id: i32,
    #[encoding("fixed_point")]
    pub x: f64,
    #[encoding("fixed_point")]
    pub y: f64,
    #[encoding("fixed_point")]
    pub z: f64,
    pub count: i16,
}
