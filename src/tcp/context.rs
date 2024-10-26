use crate::minecraft::{ClientPacket, Packet, Packets, ServerPacket};
use std::io::{Error, ErrorKind};
use tokio::sync::mpsc::Sender;

pub struct Context<'a, T = Packets> {
    pub packet: &'a mut T,
    pub client: &'a Client,
    pub server: &'a Server,
}

#[derive(Clone, Debug)]
pub struct Client {
    tx: Sender<Vec<u8>>,
}

impl Client {
    pub fn new(tx: &Sender<Vec<u8>>) -> Self {
        Self { tx: tx.clone() }
    }

    pub fn send<T: ServerPacket>(&self, packet: T) {
        let raw_packet = match packet.serialize_packet() {
            Ok(raw_packet) => raw_packet,
            Err(e) => {
                eprintln!("Failed to serialize packet: {:?}", e);
                return;
            }
        };

        let tx = self.tx.clone();
        tokio::spawn(async move {
            tx.send([raw_packet.0, raw_packet.1].concat())
                .await
                .map_err(|e| Error::new(ErrorKind::Other, e))
        });
    }
}

#[derive(Clone, Debug)]
pub struct Server {
    tx: Sender<Vec<u8>>,
}

impl Server {
    pub fn new(tx: &Sender<Vec<u8>>) -> Self {
        Self { tx: tx.clone() }
    }

    pub fn send<T: ClientPacket>(&self, packet: T) {
        let raw_packet = match packet.serialize_packet() {
            Ok(raw_packet) => raw_packet,
            Err(e) => {
                eprintln!("Failed to serialize packet: {:?}", e);
                return;
            }
        };

        let tx = self.tx.clone();
        tokio::spawn(async move {
            tx.send([raw_packet.0, raw_packet.1].concat())
                .await
                .map_err(|e| Error::new(ErrorKind::Other, e))
        });
    }
}

    }
}
