use crate::minecraft::{ClientPacket, Packets, ServerPacket};
use std::io::{Error, ErrorKind};
use std::sync::mpsc::Sender;

pub struct Context<'a, T = Packets> {
    pub packet: &'a mut T,
    pub client: Client,
    pub server: Server,
}

pub struct Client {
    pub tx: Sender<Vec<u8>>,
}

impl Client {
    pub fn send<T: ClientPacket>(&self, packet: &T) -> std::io::Result<()> {
        self.tx
            .send(packet.serialize_packet()?)
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
}

pub struct Server {
    pub tx: Sender<Vec<u8>>,
}

impl Server {
    pub fn send<T: ServerPacket>(&self, packet: &T) -> std::io::Result<()> {
        self.tx
            .send(packet.serialize_packet()?)
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
}
