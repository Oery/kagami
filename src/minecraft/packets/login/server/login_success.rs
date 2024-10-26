use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x02, server)]
pub struct LoginSuccess {
    pub uuid: String,
    pub username: String,
}
