use std::any::TypeId;
use std::collections::HashMap;
use std::io;
use std::{any::Any, fmt::Debug};

use packets::{handshake, login, play, status};

use crate::kagami::callbacks::manager::PacketCallback;
use crate::kagami::callbacks::Actions;
use crate::serialization::ToVarInt;
use crate::tcp::utils::RawPacket;
use crate::tcp::State;
use crate::{
    serialization::{deserialize, Deserialize, Serialize},
    tcp::Origin,
};

pub mod packets;
pub mod registry;

pub trait Packet: Serialize + Deserialize + Debug + Any + Send + Sync {
    fn deserialize_packet(bytes: &[u8]) -> io::Result<Self>
    where
        Self: Sized,
    {
        let packet = deserialize::<Self>(bytes)?;
        Ok(packet)
    }

    fn serialize_packet(&self) -> io::Result<RawPacket> {
        let mut raw_packet = RawPacket::default();
        let (ref mut length, ref mut data) = &mut raw_packet;

        data.push(0x00); // TODO: Implement compression
        data.push(self.get_id());

        self.serialize(data)?;

        *length = (data.len() as i32).to_varint()?;
        Ok(raw_packet)
    }

    // The origin is inferred from the module path
    // The method is implemented by a macro
    fn get_origin(&self) -> Origin;

    fn get_id(&self) -> u8;

    fn handle_callbacks(
        &mut self,
        callbacks: &HashMap<TypeId, Vec<Box<dyn PacketCallback>>>,
    ) -> std::io::Result<Actions>
    where
        Self: std::marker::Sized,
    {
        let mut action = Actions::Transfer;

        if let Some(handlers) = callbacks.get(&TypeId::of::<Self>()) {
            for callback in handlers {
                match callback.call(self)? {
                    Actions::Transfer => {}
                    Actions::Filter => {
                        action = Actions::Filter;
                        break;
                    }
                    Actions::Modify => {
                        action = Actions::Modify;
                    }
                }
            }
        }

        Ok(action)
    }
}

#[derive(Debug, PartialEq)]
pub enum Packets {
    // ================ CLIENT ================
    // Handshake
    LegacyServerListPing(packets::handshake::client::LegacyServerListPing),
    SetProtocol(packets::handshake::client::SetProtocol),
    // Login
    // EncryptionBegin(packets::login::client::EncryptionBegin),
    LoginStart(packets::login::client::LoginStart),
    // Status
    PingStart(packets::status::client::PingStart),
    ClientPing(packets::status::client::Ping),
    // Play
    ArmAnimation(packets::play::client::ArmAnimation),
    ClientChat(packets::play::client::Chat),
    ClientCommand(packets::play::client::ClientCommand),
    CloseWindow(packets::play::client::CloseWindow),
    Flying(packets::play::client::Flying),
    ClientHeldItemSlot(packets::play::client::HeldItemSlot),
    ClientKeepAlive(packets::play::client::KeepAlive),
    Look(packets::play::client::Look),
    ClientPosition(packets::play::client::Position),
    PositionAndLook(packets::play::client::PositionAndLook),
    Transaction(packets::play::client::Transaction),
    UseEntity(packets::play::client::UseEntity),
    // ================ SERVER ================
    // Login
    Compress(packets::login::server::Compress),
    Disconnect(packets::login::server::Disconnect),
    LoginSuccess(packets::login::server::LoginSuccess),
    // Status
    ServerPing(packets::status::server::Ping),
    ServerInfo(packets::status::server::ServerInfo),
    // Play
    ServerChat(packets::play::server::Chat),
    ServerKeepAlive(packets::play::server::KeepAlive),
    Login(packets::play::server::Login),
    UpdateTime(packets::play::server::UpdateTime),
    SpawnPosition(packets::play::server::SpawnPosition),
    UpdateHealth(packets::play::server::UpdateHealth),
    Respawn(packets::play::server::Respawn),
    ServerPosition(packets::play::server::Position),
    ServerHeldItemSlot(packets::play::server::HeldItemSlot),
    Bed(packets::play::server::Bed),
    Animation(packets::play::server::Animation),
    // NamedEntitySpawn(packets::play::server::NamedEntitySpawn),
    Collect(packets::play::server::Collect),

    SpawnEntityPainting(packets::play::server::SpawnEntityPainting),
    SpawnEntityExperienceOrb(packets::play::server::SpawnEntityExperienceOrb),
    EntityVelocity(packets::play::server::EntityVelocity),
    EntityDestroy(packets::play::server::EntityDestroy),
    Entity(packets::play::server::Entity),
    EntityRelativeMove(packets::play::server::EntityRelativeMove),
    EntityLook(packets::play::server::EntityLook),
    EntityMoveLook(packets::play::server::EntityMoveLook),
    EntityTeleport(packets::play::server::EntityTeleport),
    EntityHeadRotation(packets::play::server::EntityHeadRotation),
    EntityStatus(packets::play::server::EntityStatus),
    AttachEntity(packets::play::server::AttachEntity),
    EntityEffect(packets::play::server::EntityEffect),

    PlayerInfo(packets::play::server::PlayerInfo),
}

impl Packets {
    pub fn deserialize_packet(
        packet_id: i32,
        bytes: &[u8],
        state: &State,
        origin: &Origin,
    ) -> io::Result<Self> {
        match origin {
            Origin::Client => match state {
                State::HandShaking => handshake::client::parse_packet(packet_id, bytes),
                State::Status => status::client::parse_packet(packet_id, bytes),
                State::Login => login::client::parse_packet(packet_id, bytes),
                State::Play => play::client::parse_packet(packet_id, bytes),
            },

            Origin::Server => match state {
                State::Status => status::server::parse_packet(packet_id, bytes),
                State::Login => login::server::parse_packet(packet_id, bytes),
                State::Play => play::server::parse_packet(packet_id, bytes),

                State::HandShaking => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid packet id",
                )),
            },
        }
    }

    pub fn serialize_packet(packet: &Self, state: &State, origin: &Origin) -> io::Result<Vec<u8>> {
        match origin {
            Origin::Client => match state {
                State::HandShaking => handshake::client::serialize_packet(packet),
                State::Status => status::client::serialize_packet(packet),
                State::Login => login::client::serialize_packet(packet),
                State::Play => play::client::serialize_packet(packet),
            },

            Origin::Server => match state {
                State::Status => status::server::serialize_packet(packet),
                State::Login => login::server::serialize_packet(packet),
                State::Play => play::server::serialize_packet(packet),

                State::HandShaking => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid packet id",
                )),
            },
        }
    }
}

#[derive(Debug)]
pub struct GlobalPacket<'a> {
    pub packet: &'a Packets,
}

pub trait ServerPacket: Packet {}
pub trait ClientPacket: Packet {}
