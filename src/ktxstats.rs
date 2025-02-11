use std::io::{Read, Seek, SeekFrom};

use crate::qw::frame::{Command, Frame, MultiFrameInfo};
use crate::qw::hidden_message::HiddenMessage;
use anyhow::{anyhow as e, Result};
use binrw::BinRead;
pub use ktxstats::v3::KtxstatsV3;

pub fn ktxstats_v3<R>(r: &mut R) -> Result<KtxstatsV3>
where
    R: Read + Seek,
{
    let stats_str = ktxstats_string(r)?;
    KtxstatsV3::try_from(stats_str.as_str()).map_err(|err| e!(err))
}

pub fn ktxstats_string<R>(r: &mut R) -> Result<String>
where
    R: Read + Seek,
{
    // string buffer writer
    let mut content = Vec::new();

    while let Ok(frame) = Frame::read(r) {
        let current_pos = r.stream_position()?;
        let next_frame_pos = current_pos + frame.body_size as u64;

        if frame.body_size == 0 || frame.command != Command::Multiple {
            r.seek(SeekFrom::Start(next_frame_pos))?;
            continue;
        }

        while let Ok(subframe) = MultiFrameInfo::read(r) {
            match subframe.hidden_message {
                HiddenMessage::Demoinfo(info) => {
                    content.extend_from_slice(&info.content);
                }
            }
        }

        let current_pos = r.stream_position()?;

        if current_pos != next_frame_pos {
            r.seek(SeekFrom::Start(next_frame_pos))?;
        }
    }

    if !content.is_empty() {
        return Ok(quake_text::bytestr::to_unicode(&content));
    }

    Err(e!("ktxstats not found"))
}

#[cfg(test)]
mod tests {
    use std::fs::{read_to_string, File};

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_ktxstats_v3() -> Result<()> {
        let file = &mut File::open("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        let stats = ktxstats_v3(file)?;

        assert_eq!(stats.version, 3);
        assert_eq!(stats.hostname, "QUAKE.SE KTX:28502".to_string());

        Ok(())
    }

    fn strip(value: &str) -> String {
        value
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    }

    #[test]
    fn test_ktxstats_string() -> Result<()> {
        {
            let file = &mut File::open("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            let expected = strip(&read_to_string(
                "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd.ktxstats.json",
            )?);
            assert_eq!(strip(&ktxstats_string(file)?), strip(&expected));
        }
        {
            let file = &mut File::open("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            let expected = strip(&read_to_string(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd.ktxstats.json",
            )?);
            assert_eq!(strip(&ktxstats_string(file)?), strip(&expected));
        }
        {
            let file =
                &mut File::open("tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd")?;
            assert_eq!(
                ktxstats_string(file).unwrap_err().to_string(),
                "ktxstats not found"
            );
        }

        Ok(())
    }
}
