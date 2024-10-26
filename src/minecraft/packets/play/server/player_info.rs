use crate::minecraft::Packet;
use crate::serialization::{deserialize_varint, serialize_varint, Deserialize, Serialize};
use kagami_macro::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub action: PlayerInfoAction,
}

impl Packet for PlayerInfo {
    fn get_id(&self) -> u8 {
        0x38
    }

    fn get_origin(&self) -> crate::tcp::Origin {
        crate::tcp::Origin::Client
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum PlayerInfoAction {
    AddPlayer(Vec<AddPlayer>),
    UpdateGameMode(Vec<UpdateGameMode>),
    UpdatePing(Vec<UpdatePing>),
    UpdateDisplayName(Vec<UpdateDisplayName>),
    RemovePlayer(Vec<RemovePlayer>),
}

#[derive(Debug, PartialEq)]
pub struct AddPlayer {
    pub uuid: Uuid,
    pub name: String,
    pub properties: Vec<Property>,
    pub game_mode: i32, // VarInt
    pub ping: i32,      // VarInt
    pub has_display_name: bool,
    pub display_name: Option<String>,
}

impl Deserialize for AddPlayer {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let uuid = Uuid::deserialize(reader)?;
        let name = String::deserialize(reader)?;
        let properties = Vec::deserialize(reader)?;
        let game_mode = deserialize_varint(reader)?;
        let ping = deserialize_varint(reader)?;
        let has_display_name = bool::deserialize(reader)?;
        let display_name = if has_display_name {
            Some(String::deserialize(reader)?)
        } else {
            None
        };

        Ok(AddPlayer {
            uuid,
            name,
            properties,
            game_mode,
            ping,
            has_display_name,
            display_name,
        })
    }
}

impl Serialize for AddPlayer {
    fn serialize(&self, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
        self.uuid.serialize(buf)?;
        self.name.serialize(buf)?;
        self.properties.serialize(buf)?;
        serialize_varint(&self.game_mode, buf)?;
        serialize_varint(&self.ping, buf)?;
        self.has_display_name.serialize(buf)?;
        if let Some(ref display_name) = self.display_name {
            display_name.serialize(buf)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePing {
    pub uuid: Uuid,
    #[encoding("varint")]
    pub ping: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGameMode {
    pub uuid: Uuid,
    #[encoding("varint")]
    pub game_mode: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDisplayName {
    pub uuid: Uuid,
    pub has_display_name: bool,
    pub display_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RemovePlayer {
    pub uuid: Uuid,
}
