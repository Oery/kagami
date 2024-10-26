use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Serialize};

use byteorder::{ReadBytesExt, WriteBytesExt};

#[packet(0x0B, server)]
pub struct Animation {
    #[encoding("varint")]
    pub entity_id: i32,
    pub animation: AnimationKind,
}

#[derive(Debug, PartialEq)]
pub enum AnimationKind {
    SwingArm,
    TakeDamage,
    LeaveBed,
    Eat,
    CriticalEffect,
    MagicalCriticalEffect,
}

impl Deserialize for AnimationKind {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        use AnimationKind::*;

        let action = reader.read_u8()?;
        match action {
            0 => Ok(SwingArm),
            1 => Ok(TakeDamage),
            2 => Ok(LeaveBed),
            3 => Ok(Eat),
            4 => Ok(CriticalEffect),
            5 => Ok(MagicalCriticalEffect),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid animation kind : {action}"),
            )),
        }
    }
}

impl Serialize for AnimationKind {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        use AnimationKind::*;

        match self {
            SwingArm => buf.write_u8(0),
            TakeDamage => buf.write_u8(1),
            LeaveBed => buf.write_u8(2),
            Eat => buf.write_u8(3),
            CriticalEffect => buf.write_u8(4),
            MagicalCriticalEffect => buf.write_u8(5),
        }
    }
}
