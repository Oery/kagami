use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct Collect {
    #[encoding("varint")]
    collected_entity_id: i32,
    #[encoding("varint")]
    collector_entity_id: i32,
}
