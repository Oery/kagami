use crate::minecraft::{Packet, Packets};
use std::io::{Error, ErrorKind, Result};

mod chat;
mod held_item_slot;
mod keep_alive;
mod login;
mod position;
mod respawn;
mod update_health;
mod update_time;

pub use chat::*;
pub use held_item_slot::HeldItemSlot;
pub use keep_alive::KeepAlive;
pub use login::Login;
pub use position::Position;
pub use respawn::Respawn;
pub use update_health::UpdateHealth;
pub use update_time::UpdateTime;

pub fn parse_packet(packet_id: i32, bytes: &[u8]) -> Result<Packets> {
    match packet_id {
        0x00 => Ok(Packets::ServerKeepAlive(KeepAlive::deserialize_packet(
            bytes,
        )?)),

        0x01 => Ok(Packets::Login(Login::deserialize_packet(bytes)?)),
        0x02 => Ok(Packets::ServerChat(Chat::deserialize_packet(bytes)?)),
        0x03 => Ok(Packets::UpdateTime(UpdateTime::deserialize_packet(bytes)?)),
        0x06 => Ok(Packets::UpdateHealth(UpdateHealth::deserialize_packet(
            bytes,
        )?)),
        0x07 => Ok(Packets::Respawn(Respawn::deserialize_packet(bytes)?)),
        0x08 => Ok(Packets::ServerPosition(Position::deserialize_packet(
            bytes,
        )?)),
        0x09 => Ok(Packets::ServerHeldItemSlot(
            HeldItemSlot::deserialize_packet(bytes)?,
        )),

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
        Packets::UpdateTime(packet) => packet.serialize_packet(),
        Packets::UpdateHealth(packet) => packet.serialize_packet(),
        Packets::Respawn(packet) => packet.serialize_packet(),
        Packets::ServerPosition(packet) => packet.serialize_packet(),
        Packets::ServerHeldItemSlot(packet) => packet.serialize_packet(),

        _ => panic!("Invalid packet: not a server play: {:#?}", packet),
    }
}
