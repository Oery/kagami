use crate::minecraft::{AnyPacket, Packet};
use std::io::{Error, ErrorKind, Result};

mod ping;
mod ping_start;

pub use ping::Ping;
pub use ping_start::PingStart;

pub fn parse_packet(packet_id: i32, data: &[u8]) -> Result<Box<dyn AnyPacket>> {
    match packet_id {
        0 => Ok(Box::new(PingStart::deserialize(data)?)),
        1 => Ok(Box::new(Ping::deserialize(data)?)),

        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unknown packet id : {}", packet_id),
        )),
    }
}
