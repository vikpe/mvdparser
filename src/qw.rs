#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    None = 0,
    Multiple = 3,
    Single = 4,
    Stats = 5,
    All = 6,
}

impl From<&u8> for Target {
    fn from(value: &u8) -> Self {
        // parse first 3 bits
        match value & 7 {
            3 => Target::Multiple,
            4 => Target::Single,
            5 => Target::Stats,
            6 => Target::All,
            _ => Target::None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Qwd = 0,
    Read = 1,
    Set = 2,
    Empty = 7,
}

impl From<&u8> for Command {
    fn from(value: &u8) -> Self {
        // parse first 3 bits
        match value & 7 {
            0 => Command::Qwd, // should only appear in qwd
            1 | 3..=6 => Command::Read,
            2 => Command::Set,
            _ => Command::Empty,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    Bad = 0,
    Nop = 1,
    Disconnect = 2,
    UpdateStat = 3, // [byte] [byte]
    NqVersion = 4,  // [long] server version
    NqSetview = 5,  // [short] entity number
    Sound = 6,      // <see code>
    NqTime = 7,     // [float] server time
    Print = 8,      // [byte] id [string] null terminated string
    Stufftext = 9, // [string] stuffed into client's console buffer, the string should be \n terminated

    SetAngle = 10,       // [angle3] set the view angle to this absolute value
    ServerData = 11,     // [long] protocol ...
    Lightstyle = 12,     // [byte] [string]
    NqUpdateName = 13,   // [byte] [string]
    UpdateFrags = 14,    // [byte] player_number [short] frags
    NqClientdata = 15,   // <shortbits + data>
    StopSound = 16,      // <see code>
    NqUpdateColors = 17, // [byte] [byte] [byte]
    NqParticle = 18,     // [vec3] <variable>
    Damage = 19,
    SpawnStatic = 20,
    FteSpawnStatic2 = 21,
    SpawnBaseline = 22,
    TempEntity = 23,  // variable
    SetPause = 24,    // [byte] on / off
    NqSignonnum = 25, // [byte]  used for the signon sequence
    CenterPrint = 26, // [string] to put in center of the screen
    Killedmonster = 27,
    FoundSecret = 28,
    SpawnStaticSound = 29, // [coord3] [byte] samp [byte] vol [byte] aten
    Intermission = 30,     // [vec3_t] origin [vec3_t] angle
    Finale = 31,           // [string] text
    Cdtrack = 32,          // [byte] track
    Sellscreen = 33,
    Smallkick = 34,           // set client punchangle to 2
    Bigkick = 35,             // set client punchangle to 4
    UpdatePing = 36,          // [byte] player_number [short] ping
    UpdateEntertime = 37,     // [byte] [float]
    UpdateStatLong = 38,      // [byte] [long]
    Muzzleflash = 39,         // [short] entity
    UpdateUserinfo = 40,      // [byte] slot [long] uid [string] userinfo
    Download = 41,            // [short] size [size bytes]
    Playerinfo = 42,          // variable
    Nails = 43,               // [byte] num [48 bits] xyzpy 12 12 12 4 8
    ChokeCount = 44,          // [byte] packets choked
    Modellist = 45,           // [strings]
    Soundlist = 46,           // [strings]
    Packetentities = 47,      // [...]
    Deltapacketentities = 48, // [...]
    Maxspeed = 49,            // maxspeed change, for prediction
    Entgravity = 50,          // gravity change, for prediction
    Setinfo = 51,             // setinfo on a client
    Serverinfo = 52,          // serverinfo
    UpdatePl = 53,            // [byte] [byte]
    Nails2 = 54,              // [byte] num [52 bits] nxyzpy 8 12 12 12 4 8
    FteModellistshort = 60,   // [strings]
    FteSpawnbaseline2 = 66,
    EndOfDemo = 69,
    QizmoVoice = 83,
    FteVoiceChat = 84,
    Unknown = 255,
}

impl From<&u8> for MessageType {
    fn from(value: &u8) -> Self {
        match value {
            0 => MessageType::Bad,
            1 => MessageType::Nop,
            2 => MessageType::Disconnect,
            3 => MessageType::UpdateStat,
            4 => MessageType::NqVersion,
            5 => MessageType::NqSetview,
            6 => MessageType::Sound,
            7 => MessageType::NqTime,
            8 => MessageType::Print,
            9 => MessageType::Stufftext,
            10 => MessageType::SetAngle,
            11 => MessageType::ServerData,
            12 => MessageType::Lightstyle,
            13 => MessageType::NqUpdateName,
            14 => MessageType::UpdateFrags,
            15 => MessageType::NqClientdata,
            16 => MessageType::StopSound,
            17 => MessageType::NqUpdateColors,
            18 => MessageType::NqParticle,
            19 => MessageType::Damage,
            20 => MessageType::SpawnStatic,
            21 => MessageType::FteSpawnStatic2,
            22 => MessageType::SpawnBaseline,
            23 => MessageType::TempEntity,
            24 => MessageType::SetPause,
            25 => MessageType::NqSignonnum,
            26 => MessageType::CenterPrint,
            27 => MessageType::Killedmonster,
            28 => MessageType::FoundSecret,
            29 => MessageType::SpawnStaticSound,
            30 => MessageType::Intermission,
            31 => MessageType::Finale,
            32 => MessageType::Cdtrack,
            33 => MessageType::Sellscreen,
            34 => MessageType::Smallkick,
            35 => MessageType::Bigkick,
            36 => MessageType::UpdatePing,
            37 => MessageType::UpdateEntertime,
            38 => MessageType::UpdateStatLong,
            39 => MessageType::Muzzleflash,
            40 => MessageType::UpdateUserinfo,
            41 => MessageType::Download,
            42 => MessageType::Playerinfo,
            43 => MessageType::Nails,
            44 => MessageType::ChokeCount,
            45 => MessageType::Modellist,
            46 => MessageType::Soundlist,
            47 => MessageType::Packetentities,
            48 => MessageType::Deltapacketentities,
            49 => MessageType::Maxspeed,
            50 => MessageType::Entgravity,
            51 => MessageType::Setinfo,
            52 => MessageType::Serverinfo,
            53 => MessageType::UpdatePl,
            54 => MessageType::Nails2,
            60 => MessageType::FteModellistshort,
            66 => MessageType::FteSpawnbaseline2,
            69 => MessageType::EndOfDemo,
            83 => MessageType::QizmoVoice,
            84 => MessageType::FteVoiceChat,
            _ => MessageType::Unknown,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HiddenMessage {
    AntilagPosition = 0x0000, // mvdhidden_antilag_position_header_t mvdhidden_antilag_position_t*
    Usercmd = 0x0001, // <byte: playernum> <byte:dropnum> <byte: msec, vec3_t: angles, short[3]: forward side up> <byte: buttons> <byte: impulse>
    UsercmdWeapons = 0x0002, // <byte: source playernum> <int: items> <byte[4]: ammo> <byte: result> <byte*: weapon priority (nul terminated)>
    Demoinfo = 0x0003,       // <short: block#> <byte[] content>
    CommentaryTrack = 0x0004, // <byte: track#> [todo... <byte: audioformat> <string: short-name> <string: author(s)> <float: start-offset>?]
    CommentaryData = 0x0005,  // <byte: track#> [todo... format-specific]
    CommentaryTextSegment = 0x0006, // <byte: track#> [todo... <float: duration> <string: text (utf8)>]
    Dmgdone = 0x0007, // <byte: type-flags> <short: damaged ent#> <short: damaged ent#> <short: damage>
    UsercmdWeaponsSs = 0x0008, // (same format as mvdhidden_usercmd_weapons)
    UsercmdWeaponInstruction = 0x0009, // <byte: playernum> <byte: flags> <int: sequence#> <int: mode> <byte[10]: weaponlist>
    PausedDuration = 0x000A, // <byte: msec> ... actual time elapsed, not gametime (can be used to keep stream running) ... expected to be QTV only
    Extended = 0xFFFF,       // doubt we'll ever get here: read next short...
    Unknown = 0xDEAD,
}

impl From<&u16> for HiddenMessage {
    fn from(value: &u16) -> Self {
        match value {
            0x0000 => HiddenMessage::AntilagPosition,
            0x0001 => HiddenMessage::Usercmd,
            0x0002 => HiddenMessage::UsercmdWeapons,
            0x0003 => HiddenMessage::Demoinfo,
            0x0004 => HiddenMessage::CommentaryTrack,
            0x0005 => HiddenMessage::CommentaryData,
            0x0006 => HiddenMessage::CommentaryTextSegment,
            0x0007 => HiddenMessage::Dmgdone,
            0x0008 => HiddenMessage::UsercmdWeaponsSs,
            0x0009 => HiddenMessage::UsercmdWeaponInstruction,
            0x000A => HiddenMessage::PausedDuration,
            0xFFFF => HiddenMessage::Extended,
            _ => HiddenMessage::Unknown,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrintId {
    Low = 0,
    Medium = 1,
    High = 2,
    Chat = 3, // also go to chat buffer
    Unknown = 0xDEAD,
}

impl From<&u8> for PrintId {
    fn from(value: &u8) -> Self {
        match value {
            0 => PrintId::Low,
            1 => PrintId::Medium,
            2 => PrintId::High,
            3 => PrintId::Chat,
            _ => PrintId::Unknown,
        }
    }
}
