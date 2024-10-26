use crate::minecraft::{Packet, Packets};
use crate::tcp::utils::RawPacket;
use std::io::{Error, ErrorKind, Result};

mod encryption_begin;
mod login_start;

// pub use encryption_begin::EncryptionBegin;
pub use login_start::LoginStart;

pub fn parse_packet(packet_id: i32, bytes: &[u8]) -> Result<Packets> {
    match packet_id {
        0 => Ok(Packets::LoginStart(LoginStart::deserialize_packet(bytes)?)),
        // 1 => Ok(Packets::EncryptionBegin(EncryptionBegin::deserialize_packet(bytes)?)),
        _ => Err(Error::new(ErrorKind::InvalidData, "Unknown packet id")),
    }
}

pub fn serialize_packet(packet: &Packets) -> Result<RawPacket> {
    match packet {
        Packets::LoginStart(packet) => packet.serialize_packet(),
        // Packets::EncryptionBegin(packet) => packet.serialize_packet(),
        _ => panic!("Invalid packet: not a client login: {:#?}", packet),
    }
}
