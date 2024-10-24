// TODO: Will require Worlds API to be able to know the context of those.

pub struct EntityMetadata {
    pub properties: Vec<Metadata>,
}

pub enum EntityState {
    OnFire,                 // 0x01
    Crouched,               // 0x02
    Sprinting,              // 0x08
    EatingDrinkingBlocking, // 0x10
    Invisible,              // 0x20
}

pub enum ArmorStandFlags {
    Small,       // 0x01
    HasGravity,  // 0x02
    HasArms,     // 0x04
    NoBasePlate, // 0x08
    IsMarker,    // 0x10
}

pub enum SkinFlags {
    CapeEnabled,        // 0x01
    JacketEnabled,      // 0x02
    LeftSleeveEnabled,  // 0x04
    RightSleeveEnabled, // 0x08
    LeftPantsEnabled,   // 0x10
    RightPantsEnabled,  // 0x20
    HatEnabled,         // 0x40
    Unused,             // 0x80
}

pub enum HorseState {
    IsTame,         // 0x02
    HasSaddle,      // 0x04
    HasChest,       // 0x08
    IsBred,         // 0x10
    IsEating,       // 0x20
    IsRearing,      // 0x40
    HasMouthOpened, // 0x80
}

pub enum HorseType {
    Horse,    // 0
    Donkey,   // 1
    Mule,     // 2
    Zombie,   // 3
    Skeleton, // 4
}

pub enum HorseColor {
    White,     // 0
    Creamy,    // 1
    Chestnut,  // 2
    Brown,     // 3
    Black,     // 4
    Gray,      // 5
    DarkBrown, // 6
}

pub enum HorseStyle {
    None,       // 0
    White,      // 1
    Whitefield, // 2
    WhiteDots,  // 3
    BlackDots,  // 4
}

pub enum HorseArmor {
    NoArmor,      // 0
    IronArmor,    // 1
    GoldArmor,    // 2
    DiamondArmor, // 3
}

pub enum Metadata {
    EntityState(EntityState),                             //  0
    Air(i16),                                             //  1
    NameTag(String),                                      //  2
    AlwaysShowNametag(bool),                              //  3
    Silent(bool),                                         //  4
    Health(f32),                                          //  6
    PotionColor(i32),                                     //  7
    IsPotionEffectAmbient(bool),                          //  8
    ArrowsInEntity(u8),                                   //  9
    AiDisabled(bool),                                     // 15
    Age(i8),                                              // 12
    ArmorStandFlags(ArmorStandFlags),                     // 10
    HeadPosition { pitch: f32, yaw: f32, roll: f32 },     // 11
    BodyPosition { pitch: f32, yaw: f32, roll: f32 },     // 12
    LeftArmPosition { pitch: f32, yaw: f32, roll: f32 },  // 13
    RightArmPosition { pitch: f32, yaw: f32, roll: f32 }, // 14
    LeftLegPosition { pitch: f32, yaw: f32, roll: f32 },  // 15
    RightLegPosition { pitch: f32, yaw: f32, roll: f32 }, // 16
    SkinFlags(SkinFlags),                                 // 10
    AbsorptionAmount(f32),                                // 17
    Score(i32),                                           // 18
    HorseState(HorseState),                               // 16
    HorseType(HorseType),                                 // 19
    // HorseLook{color: HorseColor, style: HorseStyle},
    OwnerName(String),      // 21
    HorseArmor(HorseArmor), // 22
    IsHanging(u8),          // 23
}
