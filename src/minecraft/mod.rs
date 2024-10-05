use std::io;

use crate::serialization::{deserialize, Deserialize};

pub mod packets;

pub trait Packet: Send + Sync + Sized + 'static + std::fmt::Debug + Deserialize {
    fn deserialize<T: AsRef<[u8]>>(bytes: T) -> Result<Self, io::Error> {
        deserialize::<Self>(bytes.as_ref())
    }
}

pub trait AnyPacket: Send + Sync + std::fmt::Debug {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: Packet> AnyPacket for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
