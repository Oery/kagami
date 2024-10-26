use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x16, client)]
pub struct ClientCommand {
    #[encoding("varint")]
    pub payload: i32,
}
