use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x09, server)]
pub struct HeldItemSlot {
    pub slot: i8,
}
