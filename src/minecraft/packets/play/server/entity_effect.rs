use crate::minecraft::registry::PotionEffects;
use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityEffect {
    #[encoding("varint")]
    pub entity_id: i32,
    pub effect: PotionEffects,
    pub amplifier: i8,
    #[encoding("varint")]
    pub duration: i32,
    pub hide_particles: bool,
}
