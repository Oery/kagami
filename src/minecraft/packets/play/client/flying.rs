use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x03)]
pub struct Flying {
    pub on_ground: bool,
}
