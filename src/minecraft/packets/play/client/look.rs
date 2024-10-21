use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct Look {
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}
