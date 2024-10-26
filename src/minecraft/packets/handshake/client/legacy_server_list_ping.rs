use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0xfe)]
pub struct LegacyServerListPing {
    pub payload: u8,
}
