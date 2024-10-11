use crate::minecraft::{AnyPacket, Packet};
use std::io::{Error, ErrorKind, Result};

mod chat;
mod keep_alive;
mod login;

pub use chat::*;
pub use keep_alive::KeepAlive;
pub use login::Login;

pub fn parse_packet(packet_id: i32, data: &[u8]) -> Result<Box<dyn AnyPacket>> {
    match packet_id {
        0x00 => Ok(Box::new(KeepAlive::deserialize(data)?)),
        0x01 => Ok(Box::new(Login::deserialize(data)?)),
        0x02 => Ok(Box::new(Chat::deserialize(data)?)),

        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unknown packet id : {}", packet_id),
        )),
    }
}
