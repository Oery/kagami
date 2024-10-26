use crate::minecraft::{ClientPacket, Packet};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x01)]
pub struct Chat {
    pub message: String,
}

impl ClientPacket for Chat {}
