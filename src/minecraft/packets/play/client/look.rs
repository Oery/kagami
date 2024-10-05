use crate::minecraft::Packet;
use crate::serialization::{Deserialize, Serialize};
use kagami_macro::{Deserialize, Packet, Serialize};

#[derive(Packet, Deserialize, Debug, Serialize)]
pub struct Look {
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}
