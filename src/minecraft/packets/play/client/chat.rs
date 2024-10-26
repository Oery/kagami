use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x01, client)]
pub struct Chat {
    pub message: String,
}
