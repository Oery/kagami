use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct HeldItemSlot {
    pub slot: i8,
}
