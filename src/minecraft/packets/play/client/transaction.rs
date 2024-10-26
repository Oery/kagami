use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x0F)]
pub struct Transaction {
    pub window_id: u8,
    pub action: i16,
    pub accepted: bool,
}
