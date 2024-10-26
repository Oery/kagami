use crate::minecraft::{Packet, Packets};
use crate::tcp::utils::RawPacket;
use std::io::{Error, ErrorKind, Result};

mod ping;
mod ping_start;

pub use ping::Ping;
pub use ping_start::PingStart;

pub fn parse_packet(packet_id: i32, bytes: &[u8]) -> Result<Packets> {
    match packet_id {
        0 => Ok(Packets::PingStart(PingStart::deserialize_packet(bytes)?)),
        1 => Ok(Packets::ClientPing(Ping::deserialize_packet(bytes)?)),

        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unknown packet id : {}", packet_id),
        )),
    }
}

pub fn serialize_packet(packet: &Packets) -> Result<RawPacket> {
    match packet {
        Packets::PingStart(packet) => packet.serialize_packet(),
        Packets::ClientPing(packet) => packet.serialize_packet(),

        _ => panic!("Invalid packet: not a client status: {:#?}", packet),
    }
}
