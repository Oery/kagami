use crate::minecraft::{AnyPacket, Packet};
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
pub use use_entity::UseEntity;

pub fn parse_packet(packet_id: i32, data: &[u8]) -> Result<Box<dyn AnyPacket>> {
    match packet_id {
        0x00 => Ok(Box::new(KeepAlive::deserialize(data)?)),
        0x01 => Ok(Box::new(Chat::deserialize(data)?)),
        0x02 => Ok(Box::new(UseEntity::deserialize(data)?)),
        0x03 => Ok(Box::new(Flying::deserialize(data)?)),
        0x04 => Ok(Box::new(Position::deserialize(data)?)),
        0x05 => Ok(Box::new(Look::deserialize(data)?)),
        0x06 => Ok(Box::new(PositionAndLook::deserialize(data)?)),

        0x09 => Ok(Box::new(HeldItemSlot::deserialize(data)?)),

        0x0A => Ok(Box::new(ArmAnimation::deserialize(data)?)),

        0x0D => Ok(Box::new(CloseWindow::deserialize(data)?)),

        0x16 => Ok(Box::new(ClientCommand::deserialize(data)?)),
        0x0F => Ok(Box::new(Transaction::deserialize(data)?)),

        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unknown packet id : {}", packet_id),
        )),
    }
}
