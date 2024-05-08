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
pub enum Message {
    Bad = 0,
    Nop = 1,
    Disconnect = 2,
    UpdateStat = 3,      // [byte] [byte]
    NqVersion = 4,       // [long] server version
    NqSetview = 5,       // [short] entity number
    Sound = 6,           // <see code>
    NqTime = 7,          // [float] server time
    Print = 8,           // [byte] id [string] null terminated string
    Stufftext = 9, // [string] stuffed into client's console buffer, the string should be \n terminated
    SetAngle = 10, // [angle3] set the view angle to this absolute value
    ServerData = 11, // [long] protocol ...
    Lightstyle = 12, // [byte] [string]
    NqUpdateName = 13, // [byte] [string]
    UpdateFrags = 14, // [byte] [short]
    NqClientdata = 15, // <shortbits + data>
    StopSound = 16, // <see code>
    NqUpdateColors = 17, // [byte] [byte] [byte]
    NqParticle = 18, // [vec3] <variable>
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
    UpdatePing = 36,          // [byte] [short]
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

impl From<&u8> for Message {
    fn from(value: &u8) -> Self {
        match value {
            0 => Message::Bad,
            1 => Message::Nop,
            2 => Message::Disconnect,
            3 => Message::UpdateStat,
            4 => Message::NqVersion,
            5 => Message::NqSetview,
            6 => Message::Sound,
            7 => Message::NqTime,
            8 => Message::Print,
            9 => Message::Stufftext,
            10 => Message::SetAngle,
            11 => Message::ServerData,
            12 => Message::Lightstyle,
            13 => Message::NqUpdateName,
            14 => Message::UpdateFrags,
            15 => Message::NqClientdata,
            16 => Message::StopSound,
            17 => Message::NqUpdateColors,
            18 => Message::NqParticle,
            19 => Message::Damage,
            20 => Message::SpawnStatic,
            21 => Message::FteSpawnStatic2,
            22 => Message::SpawnBaseline,
            23 => Message::TempEntity,
            24 => Message::SetPause,
            25 => Message::NqSignonnum,
            26 => Message::CenterPrint,
            27 => Message::Killedmonster,
            28 => Message::FoundSecret,
            29 => Message::SpawnStaticSound,
            30 => Message::Intermission,
            31 => Message::Finale,
            32 => Message::Cdtrack,
            33 => Message::Sellscreen,
            34 => Message::Smallkick,
            35 => Message::Bigkick,
            36 => Message::UpdatePing,
            37 => Message::UpdateEntertime,
            38 => Message::UpdateStatLong,
            39 => Message::Muzzleflash,
            40 => Message::UpdateUserinfo,
            41 => Message::Download,
            42 => Message::Playerinfo,
            43 => Message::Nails,
            44 => Message::ChokeCount,
            45 => Message::Modellist,
            46 => Message::Soundlist,
            47 => Message::Packetentities,
            48 => Message::Deltapacketentities,
            49 => Message::Maxspeed,
            50 => Message::Entgravity,
            51 => Message::Setinfo,
            52 => Message::Serverinfo,
            53 => Message::UpdatePl,
            54 => Message::Nails2,
            60 => Message::FteModellistshort,
            66 => Message::FteSpawnbaseline2,
            69 => Message::EndOfDemo,
            83 => Message::QizmoVoice,
            84 => Message::FteVoiceChat,
            _ => Message::Unknown,
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
