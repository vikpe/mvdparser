use std::time::Duration;

use bstr::ByteSlice;

use crate::frame::Info;
use crate::qw::Message;

#[derive(PartialOrd, Ord, Eq)]
pub struct Print {
    pub timestamp_u64: u64,
    pub target: usize,
    pub bytes: Vec<u8>,
}

impl PartialEq for Print {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp_u64 == other.timestamp_u64 && self.bytes == other.bytes
    }
}

pub fn prints(data: &[u8]) -> Vec<Print> {
    let now = std::time::Instant::now();
    let mut offset = 0;
    let mut total_ms = 0;

    let mut prints: Vec<Print> = Vec::new();

    while let Ok(frame_info) = Info::try_from(&data[offset..]) {
        total_ms += frame_info.duration;

        if frame_info.body_size == 0 {
            offset += frame_info.total_size;
            continue;
        }

        let msg_type = Message::from(data[offset + frame_info.header_size]);

        if Message::Print == msg_type {
            let pfrom = offset + frame_info.header_size + 2;
            if let Some(pto) = data[pfrom..].find([0]).map(|o| o + pfrom) {
                if pto > pfrom {
                    prints.push(Print {
                        timestamp_u64: total_ms as u64,
                        target: data[offset + frame_info.header_size + 1] as usize,
                        bytes: data[pfrom..pto - 1].to_vec(),
                    });
                }
            }
        }

        offset += frame_info.total_size;
    }

    prints.dedup();

    for p in prints {
        println!(
            "{:<6} ({}) {:?}",
            Duration::from_millis(p.timestamp_u64).as_secs(),
            p.target,
            quake_text::bytestr::to_ascii(&p.bytes),
        );
    }

    println!();
    println!("### duration: {}ms", now.elapsed().as_millis());
    println!();

    vec![]
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;

    use super::*;

    #[test]
    fn test_prints() -> Result<()> {
        let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        prints(&demo_data);
        panic!();
        Ok(())
    }
}
