use crate::minecraft::{AnyPacket, Packet};
use std::io::{Error, ErrorKind, Result};

mod encryption_begin;
mod login_start;

// pub use encryption_begin::EncryptionBegin;
pub use login_start::LoginStart;

pub fn parse_packet(packet_id: i32, data: &[u8]) -> Result<Box<dyn AnyPacket>> {
    match packet_id {
        0 => Ok(Box::new(LoginStart::deserialize(data)?)),
        // 1 => Ok(Box::new(EncryptionBegin::deserialize(data)?)),
        _ => Err(Error::new(ErrorKind::InvalidData, "Unknown packet id")),
    }
}
