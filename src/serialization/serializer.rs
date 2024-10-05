use crate::serialization::serialize_varint;

use std::io::{Result, Write};

pub trait Serialize {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()>;
}

// PRIMITIVES

impl Serialize for bool {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        buf.write_all(&[if *self { 1 } else { 0 }])?;
        Ok(())
    }
}

impl Serialize for u8 {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for u16 {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for u32 {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for i8 {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for i16 {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for i32 {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for i64 {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for f32 {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for f64 {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Serialize for String {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        let bytes = &self.as_bytes();
        serialize_varint(&(bytes.len() as i32), buf)?;
        buf.write_all(bytes)?;

        Ok(())
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        if let Some(value) = self {
            value.serialize(buf)?;
        }
        Ok(())
    }
}

impl Serialize for crate::tcp::State {
    fn serialize<W: Write>(&self, buf: &mut W) -> Result<()> {
        match self {
            crate::tcp::State::HandShaking => serialize_varint(&0, buf),
            crate::tcp::State::Status => serialize_varint(&1, buf),
            crate::tcp::State::Login => serialize_varint(&2, buf),
            crate::tcp::State::Play => serialize_varint(&3, buf),
        }
    }
}

pub fn serialize<S>(item: &S) -> Result<Vec<u8>>
where
    S: Serialize,
{
    let mut writer = std::io::Cursor::new(Vec::new());
    item.serialize(&mut writer)?;
    Ok(writer.into_inner())
}
