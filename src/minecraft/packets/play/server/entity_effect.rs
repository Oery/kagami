use crate::minecraft::registry::PotionEffects;
use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityEffect {
    #[encoding("varint")]
    entity_id: i32,
    effect: PotionEffects,
    amplifier: i8,
    #[encoding("varint")]
    duration: i32,
    hide_particles: bool,
}
