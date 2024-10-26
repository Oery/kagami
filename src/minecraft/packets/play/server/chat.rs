use crate::minecraft::{Packet, ServerPacket};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x02)]
pub struct Chat {
    pub message: String,
    pub position: ChatPosition,
}

#[derive(Debug, PartialEq)]
pub enum ChatPosition {
    Chat,   // 1
    Hotbar, // 2
    System, // 4
}

impl ServerPacket for Chat {}
