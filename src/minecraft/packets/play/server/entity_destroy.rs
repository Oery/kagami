use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint_vec, serialize_varint_vec};
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct EntityDestroy {
    #[encoding("varint")]
    pub entity_ids: Vec<i32>,
}
