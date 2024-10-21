use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use crate::tcp::State;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct SetProtocol {
    #[encoding("varint")]
    pub protocol_version: i32,
    pub server_host: String,
    pub server_port: u16,
    pub next_state: State,
}
