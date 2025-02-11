use crate::qw::message::CdTrack;
use binrw::{BinRead, NullString};

#[derive(Debug, BinRead, PartialEq)]
#[br(little)]
pub struct ServerData {
    pub protocol_version: ProtocolVersion,
    pub protocol_extensions: u32,
    pub protocol: ProtocolVersion,
    pub spawn_count: u32,
    pub gamedir: NullString,
    pub time: f32,
    pub levelname: NullString,
    pub movevars: Movevars,

    #[br(pad_before = 1)]
    pub cd_track: CdTrack,

    #[br(pad_before = 1)]
    pub serverinfo_str: NullString,
}

#[derive(Debug, PartialEq, BinRead)]
#[repr(u32)]
#[br(repr = u32)]
pub enum ProtocolVersion {
    Standard = 28u32,
    Fte = u32::from_ne_bytes(*b"FTEX"),
    Fte2 = u32::from_ne_bytes(*b"FTE2"),
    Mvd1 = u32::from_ne_bytes(*b"MVD1"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qw::frame::Frame;
    use crate::qw::message::CdTrack;
    use crate::qw::message::MessageType;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::fs::File;

    #[test]
    fn test_server_data() -> Result<()> {
        {
            let file = &mut File::open("tests/files/4on4_-s-_vs_pol[dm2]20241118-2135.mvd")?;
            Frame::read(file)?;
            MessageType::read(file)?;
            let state = ServerData::read(file)?;

            assert_eq!(state.protocol_version, ProtocolVersion::Mvd1);
            assert_eq!(state.protocol_extensions, 32);
            assert_eq!(state.protocol, ProtocolVersion::Standard);
            assert_eq!(state.spawn_count, 50);
            assert_eq!(state.gamedir.to_string(), "qw".to_string());
            assert_eq!(state.time, 65.69647);
            assert_eq!(
                state.levelname.to_string(),
                "Claustrophobopolis".to_string()
            );
            assert_eq!(
                state.movevars,
                Movevars {
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
                }
            );
            assert_eq!(state.cd_track, CdTrack { track: 0 });
            assert_eq!(state.serverinfo_str.to_string(), r#"fullserverinfo "\maxfps\77\pm_ktjump\1\*version\MVDSV 1.01-dev-antilag-r402\*z_ext\511\*admin\oddjob@discord\ktxver\1.44-dev-r402\*gamedir\qw\sv_antilag\1\maxspectators\6\maxclients\8\timelimit\20\teamplay\2\deathmatch\1\mode\4on4\matchtag\eql\hostname\berlin2 KTX Server antilag #1\fpd\158\*qvm\so\*progs\so\map\dm2\status\Countdown\serverdemo\4on4_-s-_vs_pol[dm2]20241118-2135.mvd\epoch\1731965701"
"#.to_string());
        }

        Ok(())
    }
}
