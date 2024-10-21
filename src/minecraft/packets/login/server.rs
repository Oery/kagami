use crate::minecraft::{Packet, Packets};
use std::io::{Error, ErrorKind, Result};

mod compress;
mod disconnect;
mod login_success;

pub use compress::Compress;
pub use disconnect::Disconnect;
pub use login_success::LoginSuccess;

pub fn parse_packet(packet_id: i32, bytes: &[u8]) -> Result<Packets> {
    match packet_id {
        0 => Ok(Packets::Disconnect(Disconnect::deserialize_packet(bytes)?)),
        // 1 => ???
        2 => Ok(Packets::LoginSuccess(LoginSuccess::deserialize_packet(
            bytes,
        )?)),

        3 => Ok(Packets::Compress(Compress::deserialize_packet(bytes)?)),

        _ => Err(Error::new(ErrorKind::InvalidData, "Unknown packet id")),
    }
}

pub fn serialize_packet(packet: &Packets) -> Result<Vec<u8>> {
    match packet {
        Packets::Disconnect(packet) => packet.serialize_packet(),
        //
        Packets::LoginSuccess(packet) => packet.serialize_packet(),
        Packets::Compress(packet) => packet.serialize_packet(),

        _ => panic!("Invalid packet: not a server login: {:#?}", packet),
    }
}
