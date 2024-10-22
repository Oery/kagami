use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct AttachEntity {
    entity_id: i32,
    vehicle_id: i32,
    leash: bool,
}
