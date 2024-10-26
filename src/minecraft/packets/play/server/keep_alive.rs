use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x00, server)]
pub struct KeepAlive {
    #[encoding("varint")]
    pub id: i32,
}
