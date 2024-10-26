use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x1B)]
pub struct AttachEntity {
    pub entity_id: i32,
    pub vehicle_id: i32,
    pub leash: bool,
}
