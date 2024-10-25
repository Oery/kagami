use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityStatus {
    pub entity_id: i32,
    pub entity_status: i8,
}
