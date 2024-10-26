use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x04)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}
