use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x0D)]
pub struct CloseWindow {
    pub window_id: u8,
}
