use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x1A)]
pub struct EntityStatus {
    pub entity_id: i32,
    pub entity_status: i8,
}
