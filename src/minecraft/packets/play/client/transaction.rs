use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct Transaction {
    pub window_id: u8,
    pub action: i16,
    pub accepted: bool,
}
