use crate::minecraft::registry::PotionEffects;
use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x1D)]
pub struct EntityEffect {
    #[encoding("varint")]
    pub entity_id: i32,
    pub effect: PotionEffects,
    pub amplifier: i8,
    #[encoding("varint")]
    pub duration: i32,
    pub hide_particles: bool,
}
