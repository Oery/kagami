use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct AttachEntity {
    pub entity_id: i32,
    pub vehicle_id: i32,
    pub leash: bool,
}
