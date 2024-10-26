use crate::minecraft::Packet;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x0A, client)]
pub struct ArmAnimation {}
