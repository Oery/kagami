use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct UpdateHealth {
    pub health: f32,
    #[encoding("varint")]
    pub food: i32,
    pub food_saturation: f32,
}
