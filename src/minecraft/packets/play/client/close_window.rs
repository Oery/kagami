use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x0D, client)]
pub struct CloseWindow {
    pub window_id: u8,
}
