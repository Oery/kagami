use crate::kagami::callbacks::{Actions, TypeIdMap};
use crate::minecraft::{Packet, Packets};

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

#[derive(Default)]
pub struct CallbackManager {
    pub type_map: TypeIdMap,
    pub callbacks: HashMap<TypeId, Vec<Box<dyn PacketCallback>>>,
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

    pub fn handle_packet(&self, packet: &mut Packets) -> std::io::Result<Actions> {
        match packet {
            Packets::ClientChat(chat) => chat.handle_callbacks(&self.callbacks),
            Packets::Look(look) => look.handle_callbacks(&self.callbacks),
            Packets::ServerChat(chat) => chat.handle_callbacks(&self.callbacks),
        }
    }
}
