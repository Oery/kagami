use crate::minecraft::Packet;
use crate::serialization::{serialize_varint, serialize_varint_option, Deserialize, Serialize};
use kagami_macro::{Deserialize, Packet, Serialize};
use uuid::Uuid;

#[derive(Packet, Debug, PartialEq, Default, Serialize)]
pub struct PlayerInfo {
    #[encoding("varint")]
    pub action: i32,
    pub data: Vec<Player>,
}

#[derive(Debug, PartialEq, Serialize, Default)]
pub struct Player {
    pub uuid: Uuid,
    pub name: Option<String>,
    pub properties: Option<Vec<Property>>,
    #[encoding("varint")]
    pub game_mode: Option<i32>,
    #[encoding("varint")]
    pub ping: Option<i32>,
    pub has_display_name: Option<bool>,
    pub display_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>,
}
