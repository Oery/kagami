use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct UseEntity {
    #[encoding("varint")]
    pub target: i32,
    #[encoding("varint")]
    pub mouse: i32,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
}
