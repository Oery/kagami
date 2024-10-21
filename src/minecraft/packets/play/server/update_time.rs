use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Packet, Serialize};

#[packet]
pub struct UpdateTime {
    pub age: i64,
    pub time: i64,
}
