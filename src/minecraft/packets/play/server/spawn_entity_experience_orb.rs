use crate::minecraft::Packet;
use crate::serialization::{
    deserialize_fixed_point, deserialize_varint, serialize_fixed_point, serialize_varint,
};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct SpawnEntityExperienceOrb {
    #[encoding("varint")]
    entity_id: i32,
    #[encoding("fixed_point")]
    x: f64,
    #[encoding("fixed_point")]
    y: f64,
    #[encoding("fixed_point")]
    z: f64,
    count: i16,
}
