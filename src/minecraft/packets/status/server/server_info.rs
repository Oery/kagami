use crate::minecraft::Packet;
use crate::serialization::{Deserialize, Serialize};
use kagami_macro::{Deserialize, Packet, Serialize};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Player {
    pub name: String,
    pub id: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServerStatusPlayers {
    pub max: u32,
    pub online: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Vec<Player>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServerInfoPayload {
    pub version: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players: Option<ServerStatusPlayers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
}

#[derive(Packet, Deserialize, Serialize, Debug)]
pub struct ServerInfo {
    pub server_info: ServerInfoPayload,
}
