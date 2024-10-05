// use crate::minecraft::Packet;
// use crate::serialization;
// use kagami_macro::Packet;
// use serde::{Deserialize, Serialize};

// #[derive(serde::Deserialize, Debug, Serialize, serialization::Serialize)]
// pub struct Player {
//     pub name: String,
//     pub id: String,
// }

// #[derive(serde::Deserialize, Debug, Serialize)]
// pub struct ServerStatusPlayers {
//     pub max: u32,
//     pub online: u32,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub sample: Option<Vec<Player>>,
// }

// #[derive(serde::Deserialize, Debug)]
// pub struct Version {
//     pub name: String,
//     pub protocol: u32,
// }

// #[derive(serde::Deserialize, Debug)]
// pub struct ServerInfo {
//     pub version: Version,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub players: Option<ServerStatusPlayers>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub description: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub favicon: Option<String>,
// }

// impl Packet for ServerInfo {
//     fn deserialize<T: AsRef<[u8]>>(bytes: T) -> Result<Self, std::io::Error> {
//         let json = std::str::from_utf8(bytes.as_ref()).unwrap();
//         let packet = serde_json::from_str::<Self>(json).unwrap();
//         Ok(packet)
//     }
// }
