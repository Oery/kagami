use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
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
