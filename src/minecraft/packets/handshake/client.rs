use crate::minecraft::{Packet, Packets};
use std::io::{Error, ErrorKind, Result};

mod legacy_server_list_ping;
mod set_protocol;

pub use legacy_server_list_ping::LegacyServerListPing;
pub use set_protocol::SetProtocol;

pub fn parse_packet(packet_id: i32, bytes: &[u8]) -> Result<Packets> {
    match packet_id {
        0x00 => Ok(Packets::SetProtocol(SetProtocol::deserialize_packet(
            bytes,
        )?)),

        0x01 => Ok(Packets::LegacyServerListPing(
            LegacyServerListPing::deserialize_packet(bytes)?,
        )),

        _ => Err(Error::new(ErrorKind::InvalidData, "Unknown packet id")),
    }
}

pub fn serialize_packet(packet: &Packets) -> Result<Vec<u8>> {
    match packet {
        Packets::SetProtocol(packet) => packet.serialize_packet(),
        Packets::LegacyServerListPing(packet) => packet.serialize_packet(),

        _ => panic!("Invalid packet: not a client handshake: {:#?}", packet),
    }
}
