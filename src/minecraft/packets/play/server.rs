use crate::minecraft::{Packet, Packets};
use std::io::{Error, ErrorKind, Result};

mod chat;
mod keep_alive;
mod login;

pub use chat::*;
pub use keep_alive::KeepAlive;
pub use login::Login;

pub fn parse_packet(packet_id: i32, bytes: &[u8]) -> Result<Packets> {
    match packet_id {
        0x00 => Ok(Packets::ServerKeepAlive(KeepAlive::deserialize_packet(
            bytes,
        )?)),

        0x01 => Ok(Packets::Login(Login::deserialize_packet(bytes)?)),
        0x02 => Ok(Packets::ServerChat(Chat::deserialize_packet(bytes)?)),

        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unknown packet id : {}", packet_id),
        )),
    }
}

pub fn serialize_packet(packet: &Packets) -> Result<Vec<u8>> {
    match packet {
        Packets::ServerKeepAlive(packet) => packet.serialize_packet(),
        Packets::Login(packet) => packet.serialize_packet(),
        Packets::ServerChat(packet) => packet.serialize_packet(),

        _ => panic!("Invalid packet: not a server play: {:#?}", packet),
    }
}
