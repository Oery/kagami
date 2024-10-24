use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum PotionEffects {
    Speed = 1,
    Slowness = 2,
    Haste = 3,
    MiningFatigue = 4,
    Strength = 5,
    InstantHealth = 6,
    InstantDamage = 7,
    JumpBoost = 8,
    Nausea = 9,
    Regeneration = 10,
    Resistance = 11,
    FireResistance = 12,
    WaterBreathing = 13,
    Invisibility = 14,
    Blindness = 15,
    NightVision = 16,
    Hunger = 17,
    Weakness = 18,
    Poison = 19,
    Wither = 20,
    HealthBoost = 21,
    Absorption = 22,
    Saturation = 23,
}

use crate::serialization::Deserialize;
use byteorder::ReadBytesExt;
use std::io;

impl Deserialize for PotionEffects {
    fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let byte = reader.read_u8()?;
        match PotionEffects::try_from(byte) {
            Ok(potion) => Ok(potion),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid potion effect",
            )),
        }
    }
}

use crate::serialization::Serialize;
use byteorder::WriteBytesExt;

impl Serialize for PotionEffects {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        let byte = u8::from(*self);
        buf.write_u8(byte)
    }
}
