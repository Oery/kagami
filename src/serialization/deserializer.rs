use byteorder::{BigEndian, ReadBytesExt};
use std::io::{self, Read};
use uuid::Uuid;

use crate::{
    minecraft::{
        packets::{handshake, login, play, status},
        Packets,
    },
    tcp::{Origin, State},
};

use super::deserialize_varint;

pub trait Deserialize {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self>
    where
        Self: std::marker::Sized;
}

// PRIMITIVES

impl Deserialize for bool {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let byte = reader.read_u8()?;
        Ok(byte == 1)
    }
}

impl Deserialize for u8 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_u8()
    }
}

impl Deserialize for u16 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_u16::<BigEndian>()
    }
}

impl Deserialize for u32 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_u32::<BigEndian>()
    }
}

impl Deserialize for u64 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_u64::<BigEndian>()
    }
}

impl Deserialize for i8 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i8()
    }
}

impl Deserialize for i16 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i16::<BigEndian>()
    }
}

impl Deserialize for i32 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i32::<BigEndian>()
    }
}

impl Deserialize for i64 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_i64::<BigEndian>()
    }
}

impl Deserialize for f32 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_f32::<BigEndian>()
    }
}

impl Deserialize for f64 {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_f64::<BigEndian>()
    }
}

impl Deserialize for String {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let length = deserialize_varint(reader)?;

        let mut str_bytes = vec![0u8; length as usize];
        reader.read_exact(&mut str_bytes)?;

        String::from_utf8(str_bytes)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid utf8"))
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let value = T::deserialize(reader)?;
        Ok(Some(value))
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let length = deserialize_varint(reader)?;
        let mut vec = Vec::with_capacity(length as usize);
        for _ in 0..length {
            vec.push(T::deserialize(reader)?);
        }
        Ok(vec)
    }
}

impl Deserialize for Uuid {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let most_significant = reader.read_u64::<BigEndian>()?;
        let least_significant = reader.read_u64::<BigEndian>()?;
        let mut uuid_bytes = [0u8; 16];
        uuid_bytes[..8].copy_from_slice(&most_significant.to_be_bytes());
        uuid_bytes[8..].copy_from_slice(&least_significant.to_be_bytes());
        let uuid = Uuid::from_bytes(uuid_bytes);
        Ok(uuid)
    }
}

use crate::minecraft::packets::play::server::PlayerInfoAction;

impl Deserialize for PlayerInfoAction {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        use PlayerInfoAction::*;

        match deserialize_varint(reader)? {
            0 => Ok(AddPlayer(Vec::deserialize(reader)?)),
            1 => Ok(UpdateGameMode(Vec::deserialize(reader)?)),
            2 => Ok(UpdatePing(Vec::deserialize(reader)?)),
            3 => Ok(UpdateDisplayName(Vec::deserialize(reader)?)),
            4 => Ok(RemovePlayer(Vec::deserialize(reader)?)),

            action => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid player info action : {action}"),
            )),
        }
    }
}

impl Deserialize for crate::tcp::State {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        match deserialize_varint(reader)? {
            0 => Ok(crate::tcp::State::HandShaking),
            1 => Ok(crate::tcp::State::Status),
            2 => Ok(crate::tcp::State::Login),
            3 => Ok(crate::tcp::State::Play),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid state")),
        }
    }
}

use crate::minecraft::packets::play::server::ChatPosition;

impl Deserialize for ChatPosition {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        match deserialize_varint(reader)? {
            1 => Ok(ChatPosition::Chat),
            2 => Ok(ChatPosition::Hotbar),
            4 => Ok(ChatPosition::System),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid chat position",
            )),
        }
    }
}

use crate::minecraft::packets::status::server::ServerInfoPayload;

impl Deserialize for ServerInfoPayload {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let json = String::deserialize(reader)?;
        let packet = serde_json::from_str::<Self>(&json)?;
        Ok(packet)
    }
}

pub fn deserialize<D>(bytes: &[u8]) -> io::Result<D>
where
    D: Deserialize,
{
    let mut reader = std::io::Cursor::new(bytes);
    D::deserialize(&mut reader)
}

pub fn deserialize_any(
    origin: &Origin,
    state: &State,
    packet_id: i32,
    data: &[u8],
) -> io::Result<Packets> {
    match origin {
        Origin::Client => match state {
            State::HandShaking => handshake::client::parse_packet(packet_id, data),
            State::Status => status::client::parse_packet(packet_id, data),
            State::Login => login::client::parse_packet(packet_id, data),
            State::Play => play::client::parse_packet(packet_id, data),
        },

        Origin::Server => match state {
            State::Status => status::server::parse_packet(packet_id, data),
            State::Login => login::server::parse_packet(packet_id, data),
            State::Play => play::server::parse_packet(packet_id, data),

            State::HandShaking => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid packet id",
            )),
        },
    }
}
