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
        self.gen::<status::server::ServerInfo>(0x00, Status, Server);
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
        // self.gen::<play::client::BlockDig>(0x07, Play, Client);
        // self.gen::<play::client::BlockPlace>(0x08, Play, Client);
        self.gen::<play::client::HeldItemSlot>(0x09, Play, Client);
        self.gen::<play::client::ArmAnimation>(0x0A, Play, Client);
        // self.gen::<play::client::EntityAction>(0x0B, Play, Client);
        // self.gen::<play::client::SteerVehicle>(0x0C, Play, Client);
        self.gen::<play::client::CloseWindow>(0x0D, Play, Client);
        // self.gen::<play::client::WindowClick>(0x0E, Play, Client);
        self.gen::<play::client::Transaction>(0x0F, Play, Client);
        // self.gen::<play::client::SetCreativeSlot>(0x10, Play, Client);
        // self.gen::<play::client::EnchantItem>(0x11, Play, Client);
        // self.gen::<play::client::UpdateSign>(0x12, Play, Client);
        // self.gen::<play::client::Abilities>(0x13, Play, Client);
        // self.gen::<play::client::TabComplete>(0x14, Play, Client);
        // self.gen::<play::client::Settings>(0x15, Play, Client);
        self.gen::<play::client::ClientCommand>(0x16, Play, Client);
        // self.gen::<play::client::CustomPayload>(0x17, Play, Client);
        // self.gen::<play::client::Spectate>(0x18, Play, Client);
        // self.gen::<play::client::ResourcePackReceive>(0x19, Play, Client);

        // Play Server
        self.gen::<play::server::KeepAlive>(0x00, Play, Server);
        self.gen::<play::server::Login>(0x01, Play, Server);
        self.gen::<play::server::Chat>(0x02, Play, Server);
        self.gen::<play::server::UpdateTime>(0x03, Play, Server);
        // self.gen::<play::server::EntityEquipment>(0x04, Play, Server);
        // self.gen::<play::server::SpawnPosition>(0x05, Play, Server);
        self.gen::<play::server::UpdateHealth>(0x06, Play, Server);
        self.gen::<play::server::Respawn>(0x07, Play, Server);
        self.gen::<play::server::Position>(0x08, Play, Server);
        self.gen::<play::server::HeldItemSlot>(0x09, Play, Server);
        self.gen::<play::server::Bed>(0x0A, Play, Server);
        self.gen::<play::server::Animation>(0x0B, Play, Server);
        // self.gen::<play::server::NamedEntitySpawn>(0x0C, Play, Server);
        self.gen::<play::server::Collect>(0x0D, Play, Server);
        // self.gen::<play::server::SpawnEntity>(0x0E, Play, Server);
        // self.gen::<play::server::SpawnEntityLiving>(0x0F, Play, Server);
        self.gen::<play::server::SpawnEntityPainting>(0x10, Play, Server);
        self.gen::<play::server::SpawnEntityExperienceOrb>(0x11, Play, Server);
        self.gen::<play::server::EntityVelocity>(0x12, Play, Server);
        self.gen::<play::server::EntityDestroy>(0x13, Play, Server);
        self.gen::<play::server::Entity>(0x14, Play, Server);
        // self.gen::<play::server::EntityRelativeMove>(0x15, Play, Server);
        // self.gen::<play::server::EntityLook>(0x16, Play, Server);
        // self.gen::<play::server::EntityLookAndRelativeMove>(0x17, Play, Server);
        // self.gen::<play::server::EntityTeleport>(0x18, Play, Server);
        // self.gen::<play::server::EntityHeadRotation>(0x19, Play, Server);
        // self.gen::<play::server::EntityStatus>(0x1A, Play, Server);
        // self.gen::<play::server::AttachEntity>(0x1B, Play, Server);
        // self.gen::<play::server::EntityMetadata>(0x1C, Play, Server);
        // self.gen::<play::server::EntityEffect>(0x1D, Play, Server);
        // self.gen::<play::server::RemoveEntityEffect>(0x1E, Play, Server);
        // self.gen::<play::server::Experience>(0x1F, Play, Server);
        // self.gen::<play::server::UpdateAttributes>(0x20, Play, Server);
        // self.gen::<play::server::MapChunk>(0x21, Play, Server);
        // self.gen::<play::server::MultiBlockChange>(0x22, Play, Server);
        // self.gen::<play::server::BlockChange>(0x23, Play, Server);
        // self.gen::<play::server::BlockAction>(0x24, Play, Server);
        // self.gen::<play::server::BlockBreakAnimation>(0x25, Play, Server);
        // self.gen::<play::server::MapChunkBulk>(0x26, Play, Server);
        // self.gen::<play::server::Explosion>(0x27, Play, Server);
        // self.gen::<play::server::WorldEvent>(0x28, Play, Server);
        // self.gen::<play::server::NamedSoundEffect>(0x29, Play, Server);
        // self.gen::<play::server::WorldParticles>(0x2A, Play, Server);
        // self.gen::<play::server::GameStateChange>(0x2B, Play, Server);
        // self.gen::<play::server::SpawnEntityWeather>(0x2C, Play, Server);
        // self.gen::<play::server::OpenWindow>(0x2D, Play, Server);
        // self.gen::<play::server::CloseWindow>(0x2E, Play, Server);
        // self.gen::<play::server::SetSlot>(0x2F, Play, Server);
        // self.gen::<play::server::WindowItems>(0x30, Play, Server);
        // self.gen::<play::server::CraftProgressBar>(0x31, Play, Server);
        // self.gen::<play::server::Transaction>(0x32, Play, Server);
        // self.gen::<play::server::UpdateSign>(0x33, Play, Server);
        // self.gen::<play::server::Map>(0x34, Play, Server);
        // self.gen::<play::server::TileEntityData>(0x35, Play, Server);
        // self.gen::<play::server::OpenSignEntity>(0x36, Play, Server);
        // self.gen::<play::server::Statistics>(0x37, Play, Server);
        // self.gen::<play::server::PlayerInfo>(0x38, Play, Server);
        // self.gen::<play::server::Abilities>(0x39, Play, Server);
        // self.gen::<play::server::TabComplete>(0x3A, Play, Server);
        // self.gen::<play::server::ScoreboardObjective>(0x3B, Play, Server);
        // self.gen::<play::server::ScoreboardScore>(0x3C, Play, Server);
        // self.gen::<play::server::ScoreboardDisplay>(0x3D, Play, Server);
        // self.gen::<play::server::ScoreboardTeam>(0x3E, Play, Server);
        // self.gen::<play::server::CustomPayload>(0x3F, Play, Server);
        self.gen::<play::server::PlayerInfo>(0x38, Play, Server);
        // self.gen::<play::server::KickDisconnect>(0x40, Play, Server);
        // self.gen::<play::server::Difficulty>(0x41, Play, Server);
        // self.gen::<play::server::CombatEvent>(0x42, Play, Server);
        // self.gen::<play::server::Camera>(0x43, Play, Server);
        // self.gen::<play::server::WorldBorder>(0x44, Play, Server);
        // self.gen::<play::server::Title>(0x45, Play, Server);
        // self.gen::<play::server::SetCompression>(0x46, Play, Server);
        // self.gen::<play::server::PlayerListHeader>(0x47, Play, Server);
        // self.gen::<play::server::ResourcePackSend>(0x48, Play, Server);
        // self.gen::<play::server::UpdateEntityNbt>(0x49, Play, Server);
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
