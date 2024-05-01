use std::time::Duration;

use bstr::ByteSlice;

use crate::qw;

const FRAME_INFO_SIZE: usize = 6; // [time] [target/command] [size]
const FRAME_HEADER_SIZE: usize = FRAME_INFO_SIZE + 1; // [header] [cmd]
const N_MATCH_START: &[u8] = b"matchstart";
const N_MATCH_END: &[u8] = b"Standby";

pub fn countdown_duration(data: &[u8]) -> Option<Duration> {
    let offset = data.find(N_MATCH_START)?;
    Some(duration_until_offset(data, offset))
}

pub fn demo_duration(data: &[u8]) -> Option<Duration> {
    let offset = data.rfind(N_MATCH_END)?;
    Some(duration_until_offset(data, offset))
}

pub fn match_duration(data: &[u8]) -> Option<Duration> {
    let start = countdown_duration(data)?;
    let end = demo_duration(data)?;
    Some(end - start)
}

fn duration_until_offset(data: &[u8], to_offset: usize) -> Duration {
    let mut total_ms: u32 = 0;
    let mut offset = 0;

    while (offset + FRAME_HEADER_SIZE) < data.len() {
        // time [n]
        total_ms += data[offset] as u32;

        // check offset
        if offset >= to_offset {
            break;
        }

        // target/command from [n+1]
        let target = qw::Target::from(data[offset + 1]);
        let command = qw::Command::from(data[offset + 1]);

        if let qw::Target::Multiple = target {
            offset += 4; // ignore leading [0 0 0 0]
        }

        // size [n+2..]
        let frame_size: u32 = match command {
            qw::Command::Read => u32::from_le_bytes([
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
            ]),
            qw::Command::Set => 8,
            _ => 0, // should not happen
        };

        offset += FRAME_INFO_SIZE + frame_size as usize;
    }

    Duration::from_secs_f32(total_ms as f32 / 1000.0)
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_countdown_duration() -> Result<()> {
        assert_eq!(
            countdown_duration(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?),
            Some(Duration::from_secs_f32(10.156)),
        );
        assert_eq!(
            countdown_duration(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?),
            Some(Duration::from_secs_f32(10.105)),
        );
        assert_eq!(
            countdown_duration(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?),
            Some(Duration::from_secs_f32(10.113)),
        );
        Ok(())
    }

    #[test]
    fn test_demo_duration() -> Result<()> {
        assert_eq!(
            demo_duration(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?),
            Some(Duration::from_secs_f32(190.169)),
        );
        assert_eq!(
            demo_duration(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?),
            Some(Duration::from_secs_f32(610.144)),
        );
        assert_eq!(
            demo_duration(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?),
            Some(Duration::from_secs_f32(1210.142)),
        );
        Ok(())
    }

    #[test]
    fn test_match_duration() -> Result<()> {
        assert_eq!(
            match_duration(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?),
            Some(Duration::from_secs_f64(180.013006211)),
        );
        assert_eq!(
            match_duration(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?),
            Some(Duration::from_secs_f64(600.038982392)),
        );
        assert_eq!(
            match_duration(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?),
            Some(Duration::from_secs_f64(1200.028967857)),
        );
        Ok(())
    }
}
