use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x00, server)]
pub struct Disconnect {
    pub reason: String,
}
