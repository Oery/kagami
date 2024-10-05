use crate::minecraft::{AnyPacket, Packet};
use std::io::{Error, ErrorKind, Result};

mod legacy_server_list_ping;
mod set_protocol;

pub use legacy_server_list_ping::LegacyServerListPing;
pub use set_protocol::SetProtocol;

pub fn parse_packet(packet_id: i32, data: &[u8]) -> Result<Box<dyn AnyPacket>> {
    match packet_id {
        0x00 => Ok(Box::new(SetProtocol::deserialize(data)?)),
        0x01 => Ok(Box::new(LegacyServerListPing::deserialize(data)?)),
        _ => Err(Error::new(ErrorKind::InvalidData, "Unknown packet id")),
    }
}
