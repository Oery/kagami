use crate::minecraft::{Packet, Packets};
use std::io::{Error, ErrorKind, Result};

mod animation;
mod attach_entity;
mod bed;
mod chat;
mod collect;
mod entity;
mod entity_destroy;
mod entity_head_rotation;
mod entity_look;
mod entity_move_look;
mod entity_relative_move;
mod entity_status;
mod entity_teleport;
mod entity_velocity;
mod held_item_slot;
mod keep_alive;
mod login;
mod player_info;
mod position;
mod respawn;
mod spawn_entity_experience_orb;
mod spawn_entity_painting;
mod update_health;
mod update_time;

pub use animation::Animation;
pub use attach_entity::AttachEntity;
pub use bed::Bed;
pub use chat::*;
pub use collect::Collect;
pub use entity::Entity;
pub use entity_destroy::EntityDestroy;
pub use entity_head_rotation::EntityHeadRotation;
pub use entity_look::EntityLook;
pub use entity_move_look::EntityMoveLook;
pub use entity_relative_move::EntityRelativeMove;
pub use entity_status::EntityStatus;
pub use entity_teleport::EntityTeleport;
pub use entity_velocity::EntityVelocity;
pub use held_item_slot::HeldItemSlot;
pub use keep_alive::KeepAlive;
pub use login::Login;
pub use player_info::*;
pub use position::Position;
pub use respawn::Respawn;
pub use spawn_entity_experience_orb::SpawnEntityExperienceOrb;
pub use spawn_entity_painting::SpawnEntityPainting;
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

        // 0x04 => Ok(Packets::EntityEquipment(EntityEquipment::deserialize_packet(
        //     bytes,
        // )?)),

        // 0x05 => Ok(Packets::SpawnPosition(SpawnPosition::deserialize_packet(
        //     bytes,
        // )?)),
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
        0x0A => Ok(Packets::Bed(Bed::deserialize_packet(bytes)?)),
        0x0B => Ok(Packets::Animation(Animation::deserialize_packet(bytes)?)),
        // 0x0C => Ok(Packets::NamedEntitySpawn(NamedEntitySpawn::deserialize_packet(
        //     bytes,
        // )?)),
        0x0D => Ok(Packets::Collect(Collect::deserialize_packet(bytes)?)),
        // 0x0E => Ok(Packets::SpawnEntity(SpawnEntity::deserialize_packet(
        //     bytes,
        // )?)),
        // 0x0F => Ok(Packets::SpawnEntityLiving(
        //     SpawnEntityLiving::deserialize_packet(bytes)?,
        // )),
        0x10 => Ok(Packets::SpawnEntityPainting(
            SpawnEntityPainting::deserialize_packet(bytes)?,
        )),

        0x11 => Ok(Packets::SpawnEntityExperienceOrb(
            SpawnEntityExperienceOrb::deserialize_packet(bytes)?,
        )),

        0x12 => Ok(Packets::EntityVelocity(EntityVelocity::deserialize_packet(
            bytes,
        )?)),

        0x13 => Ok(Packets::EntityDestroy(EntityDestroy::deserialize_packet(
            bytes,
        )?)),

        0x14 => Ok(Packets::Entity(Entity::deserialize_packet(bytes)?)),
        0x15 => Ok(Packets::EntityRelativeMove(
            EntityRelativeMove::deserialize_packet(bytes)?,
        )),
        0x16 => Ok(Packets::EntityLook(EntityLook::deserialize_packet(bytes)?)),
        0x17 => Ok(Packets::EntityMoveLook(EntityMoveLook::deserialize_packet(
            bytes,
        )?)),
        0x18 => Ok(Packets::EntityTeleport(EntityTeleport::deserialize_packet(
            bytes,
        )?)),
        0x19 => Ok(Packets::EntityHeadRotation(
            EntityHeadRotation::deserialize_packet(bytes)?,
        )),
        0x1A => Ok(Packets::EntityStatus(EntityStatus::deserialize_packet(
            bytes,
        )?)),
        0x1B => Ok(Packets::AttachEntity(AttachEntity::deserialize_packet(
            bytes,
        )?)),

        0x38 => Ok(Packets::PlayerInfo(PlayerInfo::deserialize_packet(bytes)?)),

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
        Packets::Bed(packet) => packet.serialize_packet(),
        Packets::Animation(packet) => packet.serialize_packet(),
        // Packets::NamedEntitySpawn(packet) => packet.serialize_packet(),
        Packets::Collect(packet) => packet.serialize_packet(),

        Packets::SpawnEntityPainting(packet) => packet.serialize_packet(),
        Packets::SpawnEntityExperienceOrb(packet) => packet.serialize_packet(),
        Packets::EntityVelocity(packet) => packet.serialize_packet(),
        Packets::EntityDestroy(packet) => packet.serialize_packet(),
        Packets::Entity(packet) => packet.serialize_packet(),
        Packets::EntityRelativeMove(packet) => packet.serialize_packet(),
        Packets::EntityLook(packet) => packet.serialize_packet(),
        Packets::EntityMoveLook(packet) => packet.serialize_packet(),
        Packets::EntityTeleport(packet) => packet.serialize_packet(),
        Packets::EntityHeadRotation(packet) => packet.serialize_packet(),
        Packets::EntityStatus(packet) => packet.serialize_packet(),
        Packets::AttachEntity(packet) => packet.serialize_packet(),

        Packets::PlayerInfo(packet) => packet.serialize_packet(),

        _ => panic!("Invalid packet: not a server play: {:#?}", packet),
    }
}
