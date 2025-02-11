use crate::qw::frame::Frame;
use crate::qw::message::MessageType;
use crate::qw::message::Print;
use anyhow::Result;
use binrw::BinRead;
use std::io::{Read, Seek, SeekFrom};

pub fn prints<R>(r: &mut R) -> Result<Vec<Print>>
where
    R: Read + Seek,
{
    let mut prints: Vec<Print> = Vec::new();

    while let Ok(frame) = Frame::read(r) {
        let current_pos = r.stream_position()?;
        let next_frame_pos = current_pos + frame.body_size as u64;

        if frame.is_empty() {
            r.seek(SeekFrom::Start(next_frame_pos))?;
            continue;
        }

        while let Ok(msg_type) = MessageType::read(r) {
            if msg_type != MessageType::Print {
                break;
            }

            let print = Print::read(r)?;

            if !print.content.0.is_empty() {
                prints.push(print);
            }
        }

        let current_pos = r.stream_position()?;

        if current_pos != next_frame_pos {
            r.seek(SeekFrom::Start(next_frame_pos))?;
        }
    }

    prints.dedup();

    Ok(prints)
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use quake_text::bytestr::to_ascii;

    #[test]
    fn test_prints() -> Result<()> {
        let prints: Vec<Print> = prints(&mut File::open(
            "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd",
        )?)?;

        assert_eq!(prints.len(), 1273);
        assert_eq!(
            to_ascii(&prints[0].content.0),
            "bar.........axe is ready [oeks]_".to_string()
        );
        assert_eq!(
            to_ascii(&prints[1].content.0),
            "All players ready_".to_string()
        );
        assert_eq!(to_ascii(&prints[2].content.0), "Timer started_".to_string());
        assert_eq!(
            to_ascii(&prints[1272].content.0),
            "Statistics stored (matchtag not recognised)_".to_string()
        );

        Ok(())
    }
}
