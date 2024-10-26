use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x03)]
pub struct Compress {
    #[encoding("varint")]
    pub threshold: i32,
}
