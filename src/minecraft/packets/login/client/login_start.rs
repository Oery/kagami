use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x00)]
pub struct LoginStart {
    pub username: String,
}
