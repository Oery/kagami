use std::any::TypeId;
use std::collections::HashMap;
use std::io;
use std::{any::Any, fmt::Debug};

use packets::{handshake, login, play, status};

use crate::kagami::callbacks::manager::PacketCallback;
use crate::kagami::callbacks::Actions;
use crate::tcp::State;
use crate::{
    serialization::{deserialize, Deserialize, Serialize},
    tcp::Origin,
};

pub mod packets;

pub trait Packet: Serialize + Deserialize + Debug + Any + Send + Sync {
    fn deserialize_packet(bytes: &[u8]) -> io::Result<Self>
    where
        Self: Sized,
    {
        let packet = deserialize::<Self>(bytes)?;
        Ok(packet)
    }

    fn serialize_packet(&self) -> io::Result<Vec<u8>> {
        let mut writer = std::io::Cursor::new(Vec::new());
        self.serialize(&mut writer)?;
        Ok(writer.into_inner())
    }

    // The origin is inferred from the module path
    // The method is implemented by a macro
    fn get_origin(&self) -> Origin;

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

#[derive(Debug)]
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
    HeldItemSlot(packets::play::client::HeldItemSlot),
    ClientKeepAlive(packets::play::client::KeepAlive),
    Look(packets::play::client::Look),
    Position(packets::play::client::Position),
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
