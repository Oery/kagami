use crate::kagami::callbacks::{Actions, TypeIdMap};
use crate::minecraft::{GlobalPacket, Packet, Packets};

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait PacketCallback: Send + Sync {
    fn call(&self, packet: &mut dyn Any) -> std::io::Result<Actions>;
}

struct TypedCallback<T: Packet> {
    callback: Box<dyn Fn(&mut T) -> Actions + Send + Sync>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Packet> TypedCallback<T> {
    fn new<F>(callback: F) -> Self
    where
        F: Fn(&mut T) -> Actions + 'static + Send + Sync,
    {
        Self {
            callback: Box::new(callback),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Packet + 'static> PacketCallback for TypedCallback<T> {
    fn call(&self, packet: &mut dyn Any) -> std::io::Result<Actions> {
        match packet
            .downcast_mut::<T>()
            .map(|packet| (self.callback)(packet))
        {
            Some(action) => Ok(action),
            None => panic!("Failed to downcast packet"),
        }
    }
}

type GlobalCallback = Box<dyn Fn(&GlobalPacket) + Send + Sync>;

#[derive(Default)]
pub struct CallbackManager {
    pub type_map: TypeIdMap,
    pub callbacks: HashMap<TypeId, Vec<Box<dyn PacketCallback>>>,
    pub global_callbacks: Vec<GlobalCallback>,
}

impl CallbackManager {
    pub fn register<T, F>(&mut self, callback: F)
    where
        T: Packet + 'static,
        F: Fn(&mut T) -> Actions + 'static + Send + Sync,
    {
        let typed_callback = TypedCallback::new(callback);
        self.callbacks
            .entry(TypeId::of::<T>())
            .or_default()
            .push(Box::new(typed_callback));
    }

    pub fn register_global_callback<F>(&mut self, callback: F)
    where
        F: Fn(&GlobalPacket) + 'static + Send + Sync,
    {
        self.global_callbacks.push(Box::new(callback));
    }

    // TODO: Create a macro to generate this
    pub fn handle_packet(&self, packet: &mut Packets) -> std::io::Result<Actions> {
        match packet {
            Packets::LegacyServerListPing(p) => p.handle_callbacks(&self.callbacks),
            Packets::SetProtocol(p) => p.handle_callbacks(&self.callbacks),
            Packets::LoginStart(p) => p.handle_callbacks(&self.callbacks),
            Packets::PingStart(p) => p.handle_callbacks(&self.callbacks),
            Packets::ClientPing(p) => p.handle_callbacks(&self.callbacks),

            Packets::ClientKeepAlive(p) => p.handle_callbacks(&self.callbacks),
            Packets::ClientChat(p) => p.handle_callbacks(&self.callbacks),
            Packets::UseEntity(p) => p.handle_callbacks(&self.callbacks),
            Packets::Flying(p) => p.handle_callbacks(&self.callbacks),
            Packets::Position(p) => p.handle_callbacks(&self.callbacks),
            Packets::Look(p) => p.handle_callbacks(&self.callbacks),
            Packets::PositionAndLook(p) => p.handle_callbacks(&self.callbacks),
            Packets::HeldItemSlot(p) => p.handle_callbacks(&self.callbacks),
            Packets::ArmAnimation(p) => p.handle_callbacks(&self.callbacks),
            Packets::CloseWindow(p) => p.handle_callbacks(&self.callbacks),
            Packets::ClientCommand(p) => p.handle_callbacks(&self.callbacks),
            Packets::Transaction(p) => p.handle_callbacks(&self.callbacks),

            Packets::Compress(p) => p.handle_callbacks(&self.callbacks),
            Packets::Disconnect(p) => p.handle_callbacks(&self.callbacks),
            Packets::LoginSuccess(p) => p.handle_callbacks(&self.callbacks),

            Packets::ServerPing(p) => p.handle_callbacks(&self.callbacks),
            Packets::ServerInfo(p) => p.handle_callbacks(&self.callbacks),
            Packets::ServerKeepAlive(p) => p.handle_callbacks(&self.callbacks),
            Packets::ServerChat(p) => p.handle_callbacks(&self.callbacks),
            Packets::Login(p) => p.handle_callbacks(&self.callbacks),
        }
    }
}
