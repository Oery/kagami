use crate::minecraft::Packet;
use crate::serialization::{Deserialize, Serialize};
use kagami_macro::{Deserialize, Packet, Serialize};

#[derive(Packet, Deserialize, Debug, Serialize)]
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
