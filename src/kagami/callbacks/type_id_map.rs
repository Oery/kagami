use std::{any::TypeId, collections::HashMap};

use crate::minecraft::packets::handshake::client::*;
use crate::minecraft::packets::login;
use crate::minecraft::packets::play;
use crate::minecraft::packets::status;
use crate::tcp::{
    Origin::{self, *},
    State::{self, *},
};

// This map allows us to check if a packet has a callback registered for it without having to deserialize it
// All packets should be registered in this map

// A better way to do this could be a macro to add on the packets
// A packet could have some metadata containing the type id, state, and origin
// Then the macro could generate the code to register the packets in the map

#[derive(Clone)]
pub struct TypeIdMap {
    map: HashMap<i32, TypeId>,
}

impl TypeIdMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        // Handshake Client
        self.gen::<SetProtocol>(0x00, HandShaking, Client);
        self.gen::<LegacyServerListPing>(0xfe, HandShaking, Client);

        // Status Client
        self.gen::<status::client::PingStart>(0x00, Status, Client);
        self.gen::<status::client::Ping>(0x01, Status, Client);

        // Status Server
        // self.gen::<status::server::ServerInfo>(0x00, Status, Server);
        self.gen::<status::server::Ping>(0x01, Status, Server);

        // Login Client
        self.gen::<login::client::LoginStart>(0x00, Login, Client);
        // self.gen::<login::client::EncryptionBegin>(0x01, Login, Client);

        // Login Server
        self.gen::<login::server::Disconnect>(0x00, Login, Server);
        // self.gen::<login::server::EncryptionBegin>(0x01, Login, Server);
        self.gen::<login::server::LoginSuccess>(0x02, Login, Server);
        self.gen::<login::server::Compress>(0x03, Login, Server);

        // Play Client
        self.gen::<play::client::KeepAlive>(0x00, Play, Client);
        self.gen::<play::client::Chat>(0x01, Play, Client);
        self.gen::<play::client::UseEntity>(0x02, Play, Client);
        self.gen::<play::client::Flying>(0x03, Play, Client);
        self.gen::<play::client::Position>(0x04, Play, Client);
        self.gen::<play::client::Look>(0x05, Play, Client);
        self.gen::<play::client::PositionAndLook>(0x06, Play, Client);

        self.gen::<play::client::HeldItemSlot>(0x09, Play, Client);

        self.gen::<play::client::ArmAnimation>(0x0A, Play, Client);

        self.gen::<play::client::CloseWindow>(0x0D, Play, Client);

        self.gen::<play::client::ClientCommand>(0x16, Play, Client);
        self.gen::<play::client::Transaction>(0x0F, Play, Client);
    }

    pub fn get(&self, id: i32, state: &State, origin: &Origin) -> Option<&TypeId> {
        let hash = get_hash(id, state, origin);
        self.map.get(&hash)
    }

    fn gen<T: 'static>(&mut self, id: i32, state: State, origin: Origin) {
        let type_id = TypeId::of::<T>();
        let hash = get_hash(id, &state, &origin);

        self.map.insert(hash, type_id);
    }
}

impl Default for TypeIdMap {
    fn default() -> Self {
        let mut map = Self::new();
        map.initialize();
        map
    }
}

pub fn get_hash(packet_id: i32, state: &State, origin: &Origin) -> i32 {
    let state_value = *state as i32;
    let origin_value = *origin as i32;

    packet_id
        .wrapping_mul(31)
        .wrapping_add(state_value.wrapping_mul(7))
        .wrapping_add(origin_value)
}
