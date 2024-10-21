use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct LoginSuccess {
    pub uuid: String,
    pub username: String,
}
