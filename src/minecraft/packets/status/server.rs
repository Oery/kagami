use crate::minecraft::{Packet, Packets};
use std::io::{Error, ErrorKind, Result};

mod ping;
mod server_info;

pub use ping::Ping;
pub use server_info::*;

pub fn parse_packet(packet_id: i32, bytes: &[u8]) -> Result<Packets> {
    match packet_id {
        0 => Ok(Packets::ServerInfo(ServerInfo::deserialize_packet(bytes)?)),
        1 => Ok(Packets::ServerPing(Ping::deserialize_packet(bytes)?)),

        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unknown packet id : {}", packet_id),
        )),
    }
}

pub fn serialize_packet(packet: &Packets) -> Result<Vec<u8>> {
    match packet {
        Packets::ServerInfo(packet) => packet.serialize_packet(),
        Packets::ServerPing(packet) => packet.serialize_packet(),

        _ => panic!("Invalid packet: not a server status: {:#?}", packet),
    }
}
