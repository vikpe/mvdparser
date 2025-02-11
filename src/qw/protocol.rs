use binrw::{BinRead, BinResult, Endian, Error};
use bitflags::bitflags;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Eq, PartialEq, BinRead)]
#[repr(u32)]
#[br(repr = u32)]
pub enum ProtocolVersion {
    Fte = u32::from_ne_bytes(*b"FTEX"),
    Fte2 = u32::from_ne_bytes(*b"FTE2"),
    Mvd1 = u32::from_ne_bytes(*b"MVD1"),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ProtocolExtensions {
    Fte(FteExtensions),
    Fte2(Fte2Extensions),
    Mvd(Mvd1Extensions),
}

pub fn parse_protocol_extensions<R: Read + Seek>(
    reader: &mut R,
    _: Endian,
    _: (),
) -> BinResult<Vec<ProtocolExtensions>> {
    let mut pairs = Vec::new();

    for _ in 0..3 {
        let start_pos = reader.stream_position()?;
        let proto = match ProtocolVersion::read_le(reader) {
            Ok(val) => val,
            Err(_) => {
                reader.seek(SeekFrom::Start(start_pos))?;
                break;
            }
        };
        let bits = u32::read_le(reader)?;
        pairs.push(
            match proto {
                ProtocolVersion::Fte => FteExtensions::from_bits(bits).map(ProtocolExtensions::Fte),
                ProtocolVersion::Fte2 => {
                    Fte2Extensions::from_bits(bits).map(ProtocolExtensions::Fte2)
                }
                ProtocolVersion::Mvd1 => {
                    Mvd1Extensions::from_bits(bits).map(ProtocolExtensions::Mvd)
                }
            }
            .ok_or(Error::AssertFail {
                pos: start_pos,
                message: format!("Invalid extension bitfield for {proto:?}: 0x{bits:08X}"),
            })?,
        );
    }

    Ok(pairs)
}

bitflags! {
    #[derive(Debug, Eq, PartialEq)]
    pub struct FteExtensions: u32 {
        const SET_VIEW = 0x00000001;
        const SCALE = 0x00000002;
        const LIGHT_STYLE_COL = 0x00000004;
        const TRANS = 0x00000008; // Transparency, divided by 254 for (0.0..1.0)
        const VIEW2 = 0x00000010;
        const BULLET_ENS = 0x00000020; // Obsolete
        const ACCURATE_TIMINGS = 0x00000040; // Needed for CSQC
        const SOUND_DBL = 0x00000080; // Max of u16 sounds
        const FATNESS = 0x00000100; // Extrude an entity along its normal vector
        const HL_BSP = 0x00000200; // Half-Life stuff
        const TE_BULLET = 0x00000400; // Temporary Entity Bullet something?
        const MODEL_DBL = 0x00001000; // Max of u16 models
        const ENTITY_DBL = 0x00002000; // Max of 1024 ents instead of 512
        const ENTITY_DBL2 = 0x00004000; // Max of 2048 ents instead of 512 with `EntityDbl`
        const FLOAT_COORDS = 0x00008000; // Supports floating point origins.
        const VIEW_WEAPON = 0x00010000; // Cause an extra qbyte to be sent, and an extra list of models for vweaps.
        const Q2BSP = 0x00020000; // Quake 2 stuff
        const Q3BSP = 0x00040000; // Quake 3 stuff
        const COLOUR_MOD = 0x00080000; // RGB color tint of entities
        const SPLIT_SCREEN = 0x00100000;
        const HEXEN2 = 0x00200000; // More stats and working particle builtin.
        const SPAWN_STATIC2 = 0x00400000; // Sends an entity delta instead of a baseline.
        const CUSTOM_TEMP_EFFECTS = 0x00800000; // Supports custom temp ents.
        const PACKET_ENTITIES256 = 0x01000000; // Client can recieve 256 packet entities.
        const SHOW_PIC = 0x04000000;
        const SET_ATTACHMENT = 0x08000000; // MD3 tags (needs networking, they need to lerp).
        const CHUNKED_DOWNLOADS = 0x20000000; // Alternate file download method. Hopefully it'll give quadroupled download speed, especially on higher pings.
        const CSQC = 0x40000000; // CSQC additions
        const DPFLAGS = 0x80000000; // DarkPortal flags.
    }
}

bitflags! {
    #[derive(Debug, Eq, PartialEq)]
    pub struct Fte2Extensions: u32 {
        const PRYD_ON_CURSOR = 0x00000001;
        const VOICE_CHAT = 0x00000002;
        const SET_ANGLE_DELTA = 0x00000004;
        const REPLACEMENT_DELTAS = 0x00000008; // Weapon frame was part of the entity state. that flag is now the player's v_angle.
        const MAX_PLAYERS = 0x00000010; // Client is able to cope with more players than 32. abs max becomes 255, due to colormap issues.
        const PRED_INFO = 0x00000020; // MoveVar stats, NQ input sequences + acks.
        const NEW_SIZE_ENCODING = 0x00000040; // Richer size encoding.
        const INFO_BLOBS = 0x00000080; // Serverinfo + userinfo lengths can be MUCH higher (protocol is unbounded, but expect low sanity limits on userinfo), and contain nulls etc.
        const StunAware = 0x00000100; // Changes the netchan to biased-bigendian (so lead two bits are 1 and not stun's 0, so we don't get confused).
        const VrInputs = 0x00000200; // clc_move changes, more buttons etc. vr stuff!
        const LerpTime = 0x00000400; // fitz-bloat parity. redefines UF_16BIT as UF_LERPEND in favour of length coding.
    }
}

bitflags! {
    #[derive(Debug, Eq, PartialEq)]
    pub struct Mvd1Extensions: u32 {
        const FLOAT_COORDS = 0x00000001;  // quirky - doesn't apply to broadcasts, just players + entities. this gives more precision, but will bug out if you try using it to increase map bounds in ways that may not be immediately apparent. iirc this was added instead of fixing some inconsistent rounding.
        const HIGH_LAG_TELEPORT = 0x00000002; // specifies the reason for an svc_setangles call. the mvdsv implementation will fuck over any mods that writebyte them. we'd need to modify our preparse stuff to work around the issue.
        const SERVER_SIDE_WEAPON = 0x00000004; // looks half-baked. would be better to predict grabs clientside (oh noes! backpack knowledge!).
        const DEBUG_WEAPON = 0x00000008;
        const DEBUG_ANTILAG = 0x00000010;
        const HIDDEN_MESSAGES = 0x00000020; // mvd bloat. shouldn't be seen on actual servers.
    }
}
