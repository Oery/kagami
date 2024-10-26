use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x01, server)]
pub struct Ping {
    pub time: i64,
}
