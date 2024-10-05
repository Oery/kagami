use crate::minecraft::Packet;
use crate::serialization::{Deserialize, Serialize};
use kagami_macro::{Deserialize, Packet, Serialize};

#[derive(Packet, Deserialize, Debug, Serialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}
