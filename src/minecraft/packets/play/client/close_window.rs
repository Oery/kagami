use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct CloseWindow {
    pub window_id: u8,
}
