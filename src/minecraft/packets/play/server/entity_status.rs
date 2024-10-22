use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityStatus {
    entity_id: i32,
    entity_status: i8,
}
