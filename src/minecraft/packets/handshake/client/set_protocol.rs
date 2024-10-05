use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint, Deserialize, Serialize};
use crate::tcp::State;
use kagami_macro::{Deserialize, Packet, Serialize};

#[derive(Packet, Deserialize, Serialize, Debug)]
pub struct SetProtocol {
    #[encoding("varint")]
    pub protocol_version: i32,
    pub server_host: String,
    pub server_port: u16,
    pub next_state: State,
}
