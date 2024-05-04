use std::time::Duration;

use bstr::ByteSlice;

use crate::{ktxstats_string, qw, util};

const H_INFO_SIZE: usize = 6; // [time] [target/command] [size]
const H_CMD_SIZE: usize = H_INFO_SIZE + 1; // [info] [cmd]
const N_MATCH_START: &[u8] = b"matchdate";
const N_MATCH_END: &[u8] = b"Standby";

pub fn countdown_duration(data: &[u8]) -> Option<Duration> {
    let offset = data.find(N_MATCH_START)?;
    Some(duration_until_offset(data, offset))
}

pub fn demo_duration(data: &[u8]) -> Option<Duration> {
    let offset = data.rfind(N_MATCH_END).unwrap_or(data.len());
    Some(duration_until_offset(data, offset))
}

pub fn match_duration(data: &[u8]) -> Option<Duration> {
    match_duration_from_ktxstats(data).or_else(|| match_duration_from_seeking(data))
}

fn match_duration_from_seeking(data: &[u8]) -> Option<Duration> {
    let start = countdown_duration(data)?;
    let end = demo_duration(data)?;
    Some(end - start)
}

fn match_duration_from_ktxstats(data: &[u8]) -> Option<Duration> {
    let ktxstats_s = ktxstats_string(data)?;
    let (from, to) = util::offsets_between(ktxstats_s.as_bytes(), br#""duration": "#, b",")?;
    let duration_f: f64 = ktxstats_s[from..to].parse().ok()?;
    Some(Duration::from_secs_f64(duration_f))
}

fn duration_until_offset(data: &[u8], target_offset: usize) -> Duration {
    let mut total_ms: u32 = 0;
    let mut offset = 0;

    while (offset + H_CMD_SIZE) < data.len() {
        // time [n]
        total_ms += data[offset] as u32;

        // check offset
        if offset >= target_offset {
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

        offset += H_INFO_SIZE + frame_size as usize;
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
            countdown_duration(&read("tests/files/ffa_5[dm4]20240501-1229.mvd")?),
            Some(Duration::from_secs_f32(10.116)),
        );
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
        assert_eq!(
            countdown_duration(&read(
                "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd"
            )?),
            Some(Duration::from_secs_f32(10.112)),
        );
        Ok(())
    }

    #[test]
    fn test_demo_duration() -> Result<()> {
        assert_eq!(
            demo_duration(&read("tests/files/ffa_5[dm4]20240501-1229.mvd")?),
            Some(Duration::from_secs_f32(71.503)),
        );
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
        assert_eq!(
            demo_duration(&read(
                "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd"
            )?),
            Some(Duration::from_secs_f32(231.352)),
        );
        Ok(())
    }

    #[test]
    fn test_match_duration() -> Result<()> {
        assert_eq!(
            match_duration(&read("tests/files/ffa_5[dm4]20240501-1229.mvd")?),
            Some(Duration::from_secs(61)),
        );
        assert_eq!(
            match_duration(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?),
            Some(Duration::from_secs(180)),
        );
        assert_eq!(
            match_duration(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?),
            Some(Duration::from_secs(600)),
        );
        assert_eq!(
            match_duration(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?),
            Some(Duration::from_secs(1200)),
        );
        assert_eq!(
            match_duration(&read(
                "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd"
            )?),
            Some(Duration::from_secs_f64(221.24000454)),
        );
        Ok(())
    }
}
