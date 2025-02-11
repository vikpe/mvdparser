use crate::qw::frame::Frame;
use crate::qw::message::{Message, Print};
use binrw::io::TakeSeekExt;
use binrw::BinRead;
use std::io::{Read, Seek, SeekFrom};

#[allow(dead_code)]
pub fn parse_all<R>(r: &mut R) -> anyhow::Result<Vec<Print>>
where
    R: Read + Seek,
{
    let mut prints: Vec<Print> = Vec::new();

    let mut frame_index = 0;
    let mut frame_from = 0;

    while let Ok(frame) = Frame::read(r) {
        let mut debug = String::new();
        let body_from = r.stream_position()?;
        let next_frame_pos = body_from + frame.body_size as u64;
        let body_to = next_frame_pos - 1;

        debug.push_str(&format!("\nframe {frame_index} ({frame_from}-{body_to})\n"));

        if frame.is_empty() {
            continue;
        }

        let mut body = r.take_seek(frame.body_size as u64);
        let mut msg_index = 0;
        let mut msg_from = body.stream_position()?;

        loop {
            if let Ok(msg) = Message::read(&mut body) {
                debug.push_str(&format!(
                    "# {:03} ({}-{})\t{:?}\n",
                    msg_index,
                    msg_from,
                    body.stream_position()? - 1,
                    msg
                ));
            } else {
                // println!("{debug}");
                break;
            }
            msg_index += 1;
            msg_from = body.stream_position()?;
        }

        let body_bytes_read = body.stream_position()? - body_from;

        debug.push_str(&format!(
            "-------- parsed {} of {} bytes in frame body -------",
            body_bytes_read,
            frame.body_size
        ));

        if body_bytes_read < frame.body_size as u64 {
            println!("{debug}");
        }

        let current_pos = r.stream_position()?;

        if current_pos != next_frame_pos {
            r.seek(SeekFrom::Start(next_frame_pos))?;
        }

        frame_from = r.stream_position()?;
        frame_index += 1;

        if frame_index >= 30 {
            break;
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

    #[test]
    fn test_messages() -> Result<()> {
        parse_all(&mut File::open(
            "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd",
        )?)?;

        panic!("hehe");

        Ok(())
    }
}
