use crate::pkg::timezone;
use crate::qw::frame::{Command, Frame};
use crate::qw::message::{Message, PrintId};
use anyhow::{anyhow as e, Result};
use binrw::BinRead;
use chrono::{DateTime, Utc};
use quake_text::bytestr;
use std::io::{Read, Seek, SeekFrom};

const MATCHDATE_NEEDLE: &[u8; 11] = b"matchdate: ";

pub fn matchdate<R>(r: &mut R) -> Result<DateTime<Utc>>
where
    R: Read + Seek,
{
    let raw_str = matchdate_string(r)?;
    let fixed_str = timezone::replace_abbr_with_offset(&raw_str)?;
    Ok(DateTime::parse_from_str(&fixed_str, "%Y-%m-%d %H:%M:%S%z")?.to_utc())
}

fn matchdate_string<R>(r: &mut R) -> Result<String>
where
    R: Read + Seek,
{
    while let Ok(frame) = Frame::read(r) {
        let current_pos = r.stream_position()?;
        let next_frame_pos = current_pos + frame.body_size as u64;

        if frame.body_size == 0 || frame.command != Command::All {
            r.seek(SeekFrom::Start(next_frame_pos))?;
            continue;
        }

        while let Ok(msg) = Message::read(r) {
            match msg {
                Message::Print(print) => {
                    if print.id == PrintId::High && print.content.starts_with(MATCHDATE_NEEDLE) {
                        let strval = bytestr::to_utf8(&print.content[MATCHDATE_NEEDLE.len()..]);
                        return Ok(strval.trim_ascii_end().to_string());
                    }
                }
                _ => {}
            }
        }

        let current_pos = r.stream_position()?;

        if current_pos != next_frame_pos {
            r.seek(SeekFrom::Start(next_frame_pos))?;
        }
    }

    Err(e!("Unable to find matchdate"))
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
    use anyhow::Result;
    use chrono::DateTime;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_matchdate() -> Result<()> {
        assert_eq!(
            matchdate(&mut File::open(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?)?,
            DateTime::parse_from_rfc3339("2024-04-26T14:59:29+00:00")?.to_utc()
        );

        Ok(())
    }

    #[test]
    fn test_matchdate_string() -> Result<()> {
        assert_eq!(
            matchdate_string(&mut File::open(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?)?,
            "2024-04-26 16:59:29 CEST".to_string()
        );

        assert_eq!(
            matchdate_string(&mut File::open(
                "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd"
            )?)?,
            "2024-04-26 17:16:13 CEST".to_string()
        );

        assert_eq!(
            matchdate_string(&mut File::open(
                "tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd"
            )?)?,
            "2024-05-20 19:25:42 UTC".to_string()
        );

        Ok(())
    }
}
