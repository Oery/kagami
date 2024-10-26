use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x05)]
pub struct Look {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
