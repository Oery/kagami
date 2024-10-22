use crate::serialization::{Deserialize, Serialize};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Deserialize for Position {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: std::marker::Sized,
    {
        let val = reader.read_u64::<BigEndian>()?;

        let mut x = (val >> 38) as i32;
        let mut y = ((val >> 26) & 0xFFF) as i16;
        let mut z = (val & 0x3FFFFFF) as i32;

        // Apply sign correction
        if x >= 1 << 25 {
            x -= 1 << 26;
        }
        if y >= 1 << 11 {
            y -= 1 << 12;
        }
        if z >= 1 << 25 {
            z -= 1 << 26;
        }

        Ok(Position { x, y, z })
    }
}

impl Serialize for Position {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        let val = ((self.x as u64 & 0x3FFFFFF) << 38)
            | ((self.y as u64 & 0xFFF) << 26)
            | (self.z as u64 & 0x3FFFFFF);

        buf.write_u64::<BigEndian>(val)?;
        Ok(())
    }
}
