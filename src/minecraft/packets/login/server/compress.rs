use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint, Deserialize, Serialize};
use kagami_macro::{Deserialize, Packet, Serialize};

#[derive(Packet, Deserialize, Debug, Serialize)]
pub struct Compress {
    #[encoding("varint")]
    pub threshold: i32,
}
