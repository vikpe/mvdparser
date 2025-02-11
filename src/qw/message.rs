use crate::qw::entity::EntityDeltas;
use crate::qw::playerinfo::Playerinfo;
use crate::qw::primitives::{Angle, Coord};
use crate::qw::server_data::ServerData;
use binrw::helpers::until_exclusive;
use binrw::{BinRead, NullString};

#[derive(Clone, Debug, Eq, PartialEq, BinRead)]
#[br(little, repr = u8)]
#[repr(u8)]
pub enum MessageType {
    Bad = 0,                  //
    Nop = 1,                  //
    Disconnect = 2,           //
    UpdateStat = 3,           // [byte] [byte]
    NqVersion = 4,            // [long] server version
    NqSetview = 5,            // [short] entity number
    Sound = 6,                // [short] entity number [byte] index [byte] volume [byte] attenuation
    NqTime = 7,               // [float] server time
    Print = 8,                // [byte] id [string] null terminated string
    StuffText = 9,            // [string] null terminated string
    SetAngle = 10,            // [angle3] set the view angle to this absolute value
    ServerData = 11,          // [long] protocol ...
    Lightstyle = 12,          // [byte] [string]
    NqUpdateName = 13,        // [byte] [string]
    UpdateFrags = 14,         // [byte] player_number [short] frags
    NqClientdata = 15,        // <shortbits + data>
    StopSound = 16,           // <see code>
    NqUpdateColors = 17,      // [byte] [byte] [byte]
    NqParticle = 18,          // [vec3] <variable>
    Damage = 19,              //
    SpawnStatic = 20,         //
    SpawnStatic2 = 21,        //
    SpawnBaseline = 22,       //
    TempEntity = 23,          // variable
    SetPause = 24,            // [byte] on / off
    NqSignonnum = 25,         // [byte]  used for the signon sequence
    CenterPrint = 26,         // [string] to put in center of the screen
    KilledMonster = 27,       //
    FoundSecret = 28,         //
    SpawnStaticSound = 29,    // [coord3] [byte] samp [byte] vol [byte] aten
    Intermission = 30,        // [vec3_t] origin [vec3_t] angle
    Finale = 31,              // [string] text
    CdTrack = 32,             // [byte] track
    Sellscreen = 33,          //
    SmallKick = 34,           // set client punchangle to 2
    BigKick = 35,             // set client punchangle to 4
    UpdatePing = 36,          // [byte] player number [short] ping
    UpdateEnterTime = 37,     // [byte] player number [float] time
    UpdateStatLong = 38,      // [byte] player number [long] stat
    Muzzleflash = 39,         // [short] entity
    UpdateUserinfo = 40,      // [byte] slot [long] uid [string] userinfo
    Download = 41,            // [short] size [size bytes]
    Playerinfo = 42,          // variable
    Nails = 43,               // [byte] num [48 bits] xyzpy 12 12 12 4 8
    ChokeCount = 44,          // [byte] packets choked
    ModelList = 45,           // [strings]
    SoundList = 46,           // [strings]
    Packetentities = 47,      // [...]
    DeltaPacketEntities = 48, // [...]
    MaxSpeed = 49,            // maxspeed change, for prediction
    EntityGravity = 50,       // gravity change, for prediction
    SetInfo = 51,             // setinfo on a client
    Serverinfo = 52,          // serverinfo
    UpdatePacketLoss = 53,    // [byte] player number [byte] packet loss
    Nails2 = 54,              // [byte] num [52 bits] nxyzpy 8 12 12 12 4 8
    ModelListShort = 60,      // [strings]
    SpawnBaseline2 = 66,      //
    EndOfDemo = 69,           //
    QizmoVoice = 83,          //
    VoiceChat = 84,           //
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little, return_unexpected_error)]
#[rustfmt::skip]
pub enum Message {
    #[br(magic(0u8))] Bad,
    #[br(magic(1u8))] Nop,
    #[br(magic(2u8))] Disconnect,
    #[br(magic(3u8))] UpdateStat(UpdateStat),
    #[br(magic(4u8))] NqVersion(u32),
    #[br(magic(5u8))] NqSetview(u16),
    #[br(magic(6u8))] Sound(Sound),
    #[br(magic(7u8))] NqTime(f32),
    #[br(magic(8u8))] Print(Print),
    #[br(magic(9u8))] StuffText(NullString),
    #[br(magic(10u8))] SetAngle([Angle; 3]),
    #[br(magic(11u8))] ServerData(ServerData),
    #[br(magic(12u8))] Lightstyle(Lightstyle),
    #[br(magic(13u8))] NqUpdateName(NqUpdateName),
    #[br(magic(14u8))] UpdateFrags(UpdateFrags),
    #[br(magic(15u8))] NqClientdata,
    #[br(magic(16u8))] StopSound,
    #[br(magic(17u8))] NqUpdateColors,
    #[br(magic(18u8))] NqParticle,
    #[br(magic(19u8))] Damage,
    #[br(magic(20u8))] SpawnStatic,
    #[br(magic(21u8))] SpawnStatic2, // (EntityDeltas),
    #[br(magic(22u8))] SpawnBaseline(SpawnBaseline),
    #[br(magic(23u8))] TempEntity,
    #[br(magic(24u8))] SetPause(u8),
    #[br(magic(25u8))] NqSignonnum(u8),
    #[br(magic(26u8))] CenterPrint(NullString),
    #[br(magic(27u8))] KilledMonster,
    #[br(magic(28u8))] FoundSecret,
    #[br(magic(29u8))] SpawnStaticSound(SpawnStaticSound),
    #[br(magic(30u8))] Intermission(Intermission),
    #[br(magic(31u8))] Finale(NullString),
    #[br(magic(32u8))] CdTrack(u8),
    #[br(magic(33u8))] SellScreen,
    #[br(magic(34u8))] SmallKick,
    #[br(magic(35u8))] BigKick,
    #[br(magic(36u8))] UpdatePing(UpdatePing),
    #[br(magic(37u8))] UpdateEntertime(UpdateEntertime),
    #[br(magic(38u8))] UpdateStatLong(UpdateStatLong),
    #[br(magic(39u8))] Muzzleflash(u16),
    #[br(magic(40u8))] UpdateUserinfo(UpdateUserinfo),
    #[br(magic(41u8))] Download,
    #[br(magic(42u8))] Playerinfo(Playerinfo),
    #[br(magic(43u8))] Nails,
    #[br(magic(44u8))] ChokeCount,
    #[br(magic(45u8))] Modellist(ModelList),
    #[br(magic(46u8))] Soundlist(SoundList),
    #[br(magic(47u8))] PacketEntities,
    #[br(magic(48u8))] DeltaPacketEntities, // (EntityDeltas),
    #[br(magic(49u8))] MaxSpeed,
    #[br(magic(50u8))] EntityGravity,
    #[br(magic(51u8))] SetInfo,
    #[br(magic(52u8))] Serverinfo(Serverinfo),
    #[br(magic(53u8))] UpdatePacketLoss(UpdatePacketLoss),
    #[br(magic(54u8))] Nails2,
    #[br(magic(60u8))] ModelListShort,
    #[br(magic(66u8))] SpawnBaseline2, //(EntityDeltas),
    #[br(magic(69u8))] EndOfDemo,
    #[br(magic(83u8))] QizmoVoice,
    #[br(magic(84u8))] VoiceChat,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct Intermission {
    pub origin: [f32; 3],
    pub angle: [f32; 3],
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct Lightstyle {
    pub index: u8,
    pub style: NullString,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct ModelList {
    pub index: u8,
    #[br(parse_with = until_exclusive(|s: &NullString| s.is_empty()), pad_after=1)]
    pub filepaths: Vec<NullString>,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct NqUpdateColors {
    pub player_number: u8,
    pub colors: [u8; 3],
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct NqUpdateName {
    pub player_number: u8,
    pub name: NullString,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct Print {
    pub id: PrintId,
    pub content: NullString,
}

#[derive(Clone, Debug, PartialEq, BinRead)]
#[br(little, repr = u8)]
pub enum PrintId {
    Low = 0,
    Chat = 3, // also go to chat buffer
    Medium = 1,
    High = 2,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct Serverinfo {
    pub key: NullString,
    pub value: NullString,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct Sound {
    pub entity_number: u16,
    pub sound_index: u8,
    pub volume: u8,
    pub attenuation: u8,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct SoundList {
    pub index: u8,
    #[br(parse_with = until_exclusive(|s: &NullString| s.is_empty()), pad_after=1)]
    pub filepaths: Vec<NullString>,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct SpawnBaseline {
    pub entity_number: i16,
    pub model_index: u8,
    pub frame: u8,
    pub colormap: u8,
    pub skinnum: u8,
    pub origin: [Coord; 3],
    pub angles: [Angle; 3],
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct SpawnStaticSound {
    pub origin: [Coord; 3],
    pub index: u8,
    pub volume: u8,
    pub attenuation: u8,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct UpdateEntertime {
    pub player_number: u8,
    pub time: f32,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct UpdateFrags {
    pub player_number: u8,
    pub frags: i16,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct UpdatePing {
    pub player_number: u8,
    pub ping: i16,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct UpdatePacketLoss {
    pub player_number: u8,
    pub packet_loss: i8,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct UpdateStat {
    pub key: u8,
    pub value: u8,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct UpdateStatLong {
    pub key: u8,
    pub value: u32,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct UpdateUserinfo {
    pub player_number: u8,
    pub userid: u32,
    pub info: NullString,
}
