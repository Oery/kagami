use crate::minecraft::registry::PotionEffects;
use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x1E, server)]
pub struct RemoveEntityEffect {
    #[encoding("varint")]
    pub entity_id: i32,
    pub effect: PotionEffects,
}
