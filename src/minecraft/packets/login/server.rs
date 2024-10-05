use crate::minecraft::{AnyPacket, Packet};
use std::io::{Error, ErrorKind, Result};

mod compress;
mod disconnect;
mod login_success;

pub use compress::Compress;
pub use disconnect::Disconnect;
pub use login_success::LoginSuccess;

pub fn parse_packet(packet_id: i32, data: &[u8]) -> Result<Box<dyn AnyPacket>> {
    match packet_id {
        0 => Ok(Box::new(Disconnect::deserialize(data)?)),
        // 1 => ???
        2 => Ok(Box::new(LoginSuccess::deserialize(data)?)),
        3 => Ok(Box::new(Compress::deserialize(data)?)),
        _ => Err(Error::new(ErrorKind::InvalidData, "Unknown packet id")),
    }
}
