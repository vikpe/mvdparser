use binrw::{BinRead, NullString};

#[derive(Debug, PartialEq, BinRead)]
#[br(little, return_unexpected_error)]
#[rustfmt::skip]
pub enum HiddenMessage {
    #[br(magic(0x003u16))] Demoinfo(Demoinfo),
}

#[derive(Debug, BinRead, PartialEq)]
#[br(little)]
pub struct Demoinfo {
    pub block_number: u16,
    pub content: NullString,
}

#[derive(Clone, Debug, PartialEq, BinRead)]
#[br(repr=u16)]
#[rustfmt::skip]
pub enum HiddenMessageType {
    // AntilagPosition = 0x0000, // mvdhidden_antilag_position_header_t mvdhidden_antilag_position_t*
    // Usercmd = 0x0001, // <byte: playernum> <byte:dropnum> <byte: msec, vec3_t: angles, short[3]: forward side up> <byte: buttons> <byte: impulse>
    // UsercmdWeapons = 0x0002, // <byte: source playernum> <int: items> <byte[4]: ammo> <byte: result> <byte*: weapon priority (nul terminated)>
    Demoinfo = 0x0003,       // <short: block#> <byte[] content>
    // CommentaryTrack = 0x0004, // <byte: track#> [todo... <byte: audioformat> <string: short-name> <string: author(s)> <float: start-offset>?]
    // CommentaryData = 0x0005,  // <byte: track#> [todo... format-specific]
    // CommentaryTextSegment = 0x0006, // <byte: track#> [todo... <float: duration> <string: text (utf8)>]
    // Dmgdone = 0x0007, // <byte: type-flags> <short: damaged ent#> <short: damaged ent#> <short: damage>
    // UsercmdWeaponsSs = 0x0008, // (same format as mvdhidden_usercmd_weapons)
    // UsercmdWeaponInstruction = 0x0009, // <byte: playernum> <byte: flags> <int: sequence#> <int: mode> <byte[10]: weaponlist>
    // PausedDuration = 0x000A, // <byte: msec> ... actual time elapsed, not gametime (can be used to keep stream running) ... expected to be QTV only
    // Extended = 0xFFFF,       // doubt we'll ever get here: read next short...
}
