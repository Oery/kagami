use crate::minecraft::Packet;
use crate::serialization::{Deserialize, Serialize};
use kagami_macro::{Deserialize, Packet, Serialize};

#[derive(Packet, Deserialize, Debug, Serialize)]
pub struct Login {
    pub entity_id: i32,
    pub game_mode: u8,
    pub dimension: i8,
    pub difficulty: u8,
    pub max_players: u8,
    pub level_type: String,
    pub reduced_debug_info: bool,
}
