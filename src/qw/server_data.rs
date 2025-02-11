use crate::qw::protocol::{parse_protocol_extensions, ProtocolExtensions};
use binrw::{BinRead, BinResult, Endian, NullString};
use std::io::{Read, Seek};

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct ServerData {
    #[br(parse_with = parse_protocol_extensions)]
    pub extensions: Vec<ProtocolExtensions>,
    pub version: u32,
    pub spawn_count: u32,
    pub gamedir: NullString,
    pub time: f32,
    pub levelname: NullString,
    pub movevars: Movevars,

    #[br(pad_before = 1)]
    pub cd_track: u8,

    #[br(pad_before = 1, parse_with=parse_fullserverinfo)]
    pub serverinfo: NullString,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct Movevars {
    pub gravity: f32,
    pub stopspeed: f32,
    pub maxspeed: f32,
    pub spectatormaxspeed: f32,
    pub accelerate: f32,
    pub airaccelerate: f32,
    pub wateraccelerate: f32,
    pub friction: f32,
    pub waterfriction: f32,
    pub entgravity: f32,
}

pub fn parse_fullserverinfo<R: Read + Seek>(
    reader: &mut R,
    _: Endian,
    _: (),
) -> BinResult<NullString> {
    let full = NullString::read_options(reader, Endian::Little, ())?;
    const PREFIX: &[u8; 16] = b"fullserverinfo \"";
    const SUFFIX: &[u8; 2] = b"\"\n";
    const MIN_LEN: usize = PREFIX.len() + SUFFIX.len();

    if full.0.len() <= (MIN_LEN) {
        return Ok(NullString::from(""));
    }

    let serverinfo = full.0[PREFIX.len()..full.0.len() - SUFFIX.len()].to_vec();
    Ok(NullString(serverinfo))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qw::frame::Frame;
    use crate::qw::message::MessageType;
    use crate::qw::protocol::Mvd1Extensions;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::fs::File;

    #[test]
    fn test_server_data() -> Result<()> {
        {
            let file = &mut File::open("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            Frame::read(file)?;
            MessageType::read(file)?;

            assert_eq!(
                ServerData::read(file)?,
                ServerData {
                    extensions: vec![ProtocolExtensions::Mvd(Mvd1Extensions::HIDDEN_MESSAGES)],
                    version: 28,
                    spawn_count: 1138,
                    gamedir: NullString::from("qw"),
                    time: 60.211918,
                    levelname: NullString::from("Bravado - by foogs [remake]"),
                    movevars: Movevars {
                        gravity: 800.0,
                        stopspeed: 100.0,
                        maxspeed: 320.0,
                        spectatormaxspeed: 500.0,
                        accelerate: 10.0,
                        airaccelerate: 10.0,
                        wateraccelerate: 10.0,
                        friction: 4.0,
                        waterfriction: 4.0,
                        entgravity: 1.0,
                    },
                    cd_track: 0,
                    serverinfo: NullString::from(
                        r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\*admin\suom1 <suom1@irc.ax>\ktxver\1.42\sv_antilag\2\maxspectators\12\*gamedir\qw\timelimit\10\deathmatch\3\mode\1on1\hostname\QUAKE.SE KTX:28501\fpd\142\*qvm\so\*progs\so\maxclients\2\map\bravado\status\Countdown\serverdemo\duel_holy_vs_dago[bravado]20240426-1659.mvd"#
                    ),
                }
            );
        }

        {
            let file = &mut File::open("tests/files/4on4_-s-_vs_pol[dm2]20241118-2135.mvd")?;
            Frame::read(file)?;
            MessageType::read(file)?;

            assert_eq!(
                ServerData::read(file)?,
                ServerData {
                    extensions: vec![ProtocolExtensions::Mvd(Mvd1Extensions::HIDDEN_MESSAGES)],
                    version: 28,
                    spawn_count: 50,
                    gamedir: NullString::from("qw"),
                    time: 65.69647,
                    levelname: NullString::from("Claustrophobopolis"),
                    movevars: Movevars {
                        gravity: 800.0,
                        stopspeed: 100.0,
                        maxspeed: 320.0,
                        spectatormaxspeed: 500.0,
                        accelerate: 10.0,
                        airaccelerate: 10.0,
                        wateraccelerate: 10.0,
                        friction: 4.0,
                        waterfriction: 4.0,
                        entgravity: 1.0,
                    },
                    cd_track: 0,
                    serverinfo: NullString::from(
                        r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 1.01-dev-antilag-r402\*z_ext\511\*admin\oddjob@discord\ktxver\1.44-dev-r402\*gamedir\qw\sv_antilag\1\maxspectators\6\maxclients\8\timelimit\20\teamplay\2\deathmatch\1\mode\4on4\matchtag\eql\hostname\berlin2 KTX Server antilag #1\fpd\158\*qvm\so\*progs\so\map\dm2\status\Countdown\serverdemo\4on4_-s-_vs_pol[dm2]20241118-2135.mvd\epoch\1731965701"#
                    ),
                }
            );
        }

        Ok(())
    }
}
