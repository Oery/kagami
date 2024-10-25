use crate::minecraft::Packet;
use crate::serialization::Position;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct SpawnPosition {
    position: Position,
}
