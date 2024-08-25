use std::time::Duration;

use anyhow::{anyhow as e, Result};
use bstr::ByteSlice;

use crate::qw::frame;
use crate::{bytesextra, ktxstats_string, matchdate, serverinfo};

pub fn countdown_duration(data: &[u8]) -> Result<Duration> {
    if serverinfo(data).is_ok_and(|s| s.mode == Some("hoonymode".to_string())) {
        return Ok(Duration::ZERO);
    }

    let Some(offset) = data.find(matchdate::MATCHDATE_NEEDLE) else {
        return Err(e!("Countdown not found"));
    };
    Ok(duration_until_offset(data, offset))
}

pub fn demo_duration(data: &[u8]) -> Result<Duration> {
    const NEEDLE_MATCH_OVER: [u8; 0x11] = [
        0x54, 0x68, 0x65, 0x20, 0x6D, 0x61, 0x74, 0x63, 0x68, 0x20, 0x69, 0x73, 0x20, 0x6F, 0x76,
        0x65, 0x72, // "The match is over"
    ];
    const NEEDLE_STANDBY: [u8; 0x10] = [
        0x34, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x00, 0x53, 0x74, 0x61, 0x6E, 0x64, 0x62, 0x79,
        0x00, // "[serverinfo] [status] Standby"
    ];
    let offset = data
        .rfind(NEEDLE_MATCH_OVER)
        .unwrap_or_else(|| data.rfind(NEEDLE_STANDBY).unwrap_or(data.len()));
    Ok(duration_until_offset(data, offset))
}

pub fn match_duration(data: &[u8]) -> Result<Duration> {
    if serverinfo(data).is_ok_and(|s| s.mode == Some("hoonymode".to_string())) {
        return match_duration_from_seeking(data);
    }

    match_duration_from_ktxstats(data).or_else(|_| match_duration_from_seeking(data))
}

pub fn match_duration_from_seeking(data: &[u8]) -> Result<Duration> {
    let end = demo_duration(data)?;
    let begin = countdown_duration(data)?;
    Ok(end - begin)
}

pub fn match_duration_from_ktxstats(data: &[u8]) -> Result<Duration> {
    let ktxstats_s = ktxstats_string(data)?;
    let Some((from, to)) =
        bytesextra::offsets_between(ktxstats_s.as_bytes(), br#""duration": "#, b",")
    else {
        return Err(e!("Duration not found in ktxstats"));
    };
    let duration_f: f64 = ktxstats_s[from..to].parse()?;
    Ok(Duration::from_secs_f64(duration_f))
}

fn duration_until_offset(data: &[u8], target_offset: usize) -> Duration {
    let mut index = 0;
    let mut total_ms: u32 = 0;

    while let Ok(frame_info) = frame::Info::from_data_and_index(data, index) {
        if index >= target_offset {
            break;
        }

        total_ms += frame_info.duration;
        index += frame_info.size;
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
                "tests/files/1on1_milton_vs_mushi[tron]20240616-1719.mvd"
            )?)?,
            Duration::ZERO,
        );
        assert_eq!(
            countdown_duration(&read("tests/files/ffa_5[dm4]20240501-1229.mvd")?)?.as_secs(),
            10,
        );
        assert_eq!(
            countdown_duration(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?
            .as_secs(),
            10,
        );
        assert_eq!(
            countdown_duration(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?)?
            .as_secs(),
            10,
        );
        assert_eq!(
            countdown_duration(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?)?
                .as_secs(),
            10,
        );
        assert_eq!(
            countdown_duration(&read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?)?
                .as_secs(),
            10,
        );
        assert_eq!(
            countdown_duration(&read(
                "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd"
            )?)?
            .as_secs(),
            10,
        );
        Ok(())
    }

    #[test]
    fn test_demo_duration() -> Result<()> {
        assert_eq!(
            demo_duration(&read(
                "tests/files/1on1_milton_vs_mushi[tron]20240616-1719.mvd"
            )?)?
            .as_secs(),
            174,
        );
        assert_eq!(
            demo_duration(&read("tests/files/ffa_5[dm4]20240501-1229.mvd")?)?.as_secs(),
            71,
        );
        assert_eq!(
            demo_duration(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?
            .as_secs(),
            190,
        );
        assert_eq!(
            demo_duration(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?)?
            .as_secs(),
            610,
        );
        assert_eq!(
            demo_duration(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?)?.as_secs(),
            1210,
        );
        assert_eq!(
            demo_duration(&read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?)?.as_secs(),
            610,
        );
        assert_eq!(
            demo_duration(&read(
                "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd"
            )?)?
            .as_secs(),
            231,
        );
        Ok(())
    }

    #[test]
    fn test_match_duration() -> Result<()> {
        assert_eq!(
            match_duration(&read("tests/files/ffa_5[dm4]20240501-1229.mvd")?)?.as_secs(),
            61,
        );
        assert_eq!(
            match_duration(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?
            .as_secs(),
            180,
        );
        assert_eq!(
            match_duration(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?)?
            .as_secs(),
            600
        );
        assert_eq!(
            match_duration(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?)?.as_secs(),
            1200
        );
        assert_eq!(
            match_duration(&read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?)?.as_secs(),
            600
        );
        assert_eq!(
            match_duration(&read(
                "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd"
            )?)?
            .as_secs(),
            221
        );
        Ok(())
    }
}
