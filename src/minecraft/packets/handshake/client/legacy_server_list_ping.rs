use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct LegacyServerListPing {
    pub payload: u8,
}
