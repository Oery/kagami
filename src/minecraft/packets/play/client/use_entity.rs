use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint};
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x02)]
pub struct UseEntity {
    #[encoding("varint")]
    pub target: i32,
    pub interaction_type: UseEntityType,
}

#[derive(Debug, PartialEq)]
pub enum UseEntityType {
    Interact,
    Attack,
    InteractAt { x: f32, y: f32, z: f32 },
}

impl Deserialize for UseEntityType {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        use UseEntityType::*;

        let action = deserialize_varint(reader)?;
        match action {
            0 => Ok(Interact),
            1 => Ok(Attack),
            2 => Ok(InteractAt {
                x: f32::deserialize(reader)?,
                y: f32::deserialize(reader)?,
                z: f32::deserialize(reader)?,
            }),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid use entity type : {action}"),
            )),
        }
    }
}

impl Serialize for UseEntityType {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        use UseEntityType::*;

        match self {
            Interact => serialize_varint(&0, buf),
            Attack => serialize_varint(&1, buf),
            InteractAt { x, y, z } => {
                serialize_varint(&2, buf)?;
                x.serialize(buf)?;
                y.serialize(buf)?;
                z.serialize(buf)?;
                Ok(())
            }
        }
    }
}
