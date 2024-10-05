use crate::minecraft::Packet;
use crate::serialization::{Deserialize, Serialize};
use kagami_macro::{Deserialize, Packet, Serialize};

#[derive(Packet, Deserialize, Debug, Serialize)]
pub struct Transaction {
    pub window_id: u8,
    pub action: i16,
    pub accepted: bool,
}
