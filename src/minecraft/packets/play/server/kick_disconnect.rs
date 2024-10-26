use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x40, server)]
pub struct KickDisconnect {
    pub reason: String,
}
