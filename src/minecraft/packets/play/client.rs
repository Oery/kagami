use crate::minecraft::{Packet, Packets};
use crate::tcp::utils::RawPacket;
use std::io::{Error, ErrorKind, Result};

mod arm_animation;
mod chat;
mod client_command;
mod close_window;
mod flying;
mod held_item_slot;
mod keep_alive;
mod look;
mod position;
mod position_and_look;
mod transaction;
mod use_entity;

pub use arm_animation::ArmAnimation;
pub use chat::Chat;
pub use client_command::ClientCommand;
pub use close_window::CloseWindow;
pub use flying::Flying;
pub use held_item_slot::HeldItemSlot;
pub use keep_alive::KeepAlive;
pub use look::Look;
pub use position::Position;
pub use position_and_look::PositionAndLook;
pub use transaction::Transaction;
pub use use_entity::*;

pub fn parse_packet(packet_id: i32, bytes: &[u8]) -> Result<Packets> {
    match packet_id {
        0x00 => Ok(Packets::ClientKeepAlive(KeepAlive::deserialize_packet(
            bytes,
        )?)),

        0x01 => Ok(Packets::ClientChat(Chat::deserialize_packet(bytes)?)),
        0x02 => Ok(Packets::UseEntity(UseEntity::deserialize_packet(bytes)?)),
        0x03 => Ok(Packets::Flying(Flying::deserialize_packet(bytes)?)),
        0x04 => Ok(Packets::ClientPosition(Position::deserialize_packet(
            bytes,
        )?)),
        0x05 => Ok(Packets::Look(Look::deserialize_packet(bytes)?)),

        0x06 => Ok(Packets::PositionAndLook(
            PositionAndLook::deserialize_packet(bytes)?,
        )),

        0x09 => Ok(Packets::ClientHeldItemSlot(
            HeldItemSlot::deserialize_packet(bytes)?,
        )),

        0x0A => Ok(Packets::ArmAnimation(ArmAnimation::deserialize_packet(
            bytes,
        )?)),

        0x0D => Ok(Packets::CloseWindow(CloseWindow::deserialize_packet(
            bytes,
        )?)),

        0x16 => Ok(Packets::ClientCommand(ClientCommand::deserialize_packet(
            bytes,
        )?)),

        0x0F => Ok(Packets::Transaction(Transaction::deserialize_packet(
            bytes,
        )?)),

        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unknown packet id : {}", packet_id),
        )),
    }
}

pub fn serialize_packet(packet: &Packets) -> Result<RawPacket> {
    match packet {
        Packets::ClientKeepAlive(packet) => packet.serialize_packet(),
        Packets::ClientChat(packet) => packet.serialize_packet(),
        Packets::UseEntity(packet) => packet.serialize_packet(),
        Packets::Flying(packet) => packet.serialize_packet(),
        Packets::ClientPosition(packet) => packet.serialize_packet(),
        Packets::Look(packet) => packet.serialize_packet(),
        Packets::PositionAndLook(packet) => packet.serialize_packet(),
        Packets::ClientHeldItemSlot(packet) => packet.serialize_packet(),
        Packets::ArmAnimation(packet) => packet.serialize_packet(),
        Packets::CloseWindow(packet) => packet.serialize_packet(),
        Packets::ClientCommand(packet) => packet.serialize_packet(),
        Packets::Transaction(packet) => packet.serialize_packet(),

        _ => panic!("Invalid packet: not a client play: {:#?}", packet),
    }
}
