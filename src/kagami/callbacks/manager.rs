use crate::kagami::callbacks::type_id_map::TypeIdMap;
use crate::minecraft::AnyPacket;
use crate::serialization::deserialize_any;
use crate::tcp::{Origin, State};

use std::any::TypeId;
use std::collections::HashMap;

type CallbackFn = Box<dyn Fn(&dyn AnyPacket) + Sync + Send>;

#[derive(Default)]
pub struct CallbackManager {
    pub type_map: TypeIdMap,
    pub callbacks: HashMap<TypeId, Vec<CallbackFn>>,
}

impl CallbackManager {
    pub fn register<T: AnyPacket + 'static>(
        &mut self,
        callback: impl Fn(&T) + 'static + Sync + Send,
    ) {
        let boxed_callback = Box::new(move |packet: &dyn AnyPacket| {
            if let Some(concrete_packet) = packet.as_any().downcast_ref::<T>() {
                callback(concrete_packet);
            }
        });

        self.callbacks
            .entry(TypeId::of::<T>())
            .or_default()
            .push(boxed_callback);
    }

    pub fn handle_packet(&self, packet_id: i32, data: &[u8], origin: &Origin, state: &State) {
        if let Some(type_id) = self.type_map.get(packet_id, state, origin) {
            if let Some(callbacks) = self.callbacks.get(type_id) {
                let packet = deserialize_any(origin, state, packet_id, data).unwrap();
                for callback in callbacks {
                    callback(&*packet);
                }
            }
        }
    }
}
