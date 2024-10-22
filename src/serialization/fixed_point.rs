use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

// TODO: Add support for newer versions of Minecraft

pub fn serialize_fixed_point(x: &f64, writer: &mut dyn Write) -> io::Result<()> {
    let x_fixed = (x * (1 << 5) as f64) as i32;
    writer.write_i32::<BigEndian>(x_fixed)?;
    Ok(())
}

pub fn deserialize_fixed_point<R: Read>(reader: &mut R) -> io::Result<f64> {
    let x_fixed = reader.read_i32::<BigEndian>()?;
    Ok(x_fixed as f64 / (1 << 5) as f64)
}
