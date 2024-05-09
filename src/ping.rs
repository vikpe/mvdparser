use std::collections::HashMap;
use std::io::Cursor;

use anyhow::Result;

use crate::frame;
use crate::mvd::message::io::ReadMessages;
use crate::mvd::message::update_ping::ReadUpdatePing;
use crate::qw::MessageType;

pub fn pings(data: &[u8]) -> Result<HashMap<u8, u32>> {
    let max_samples: usize = 8;
    let mut sample_count: usize = 0;
    let mut index = 0;
    let mut total_pings: HashMap<u8, Vec<u16>> = HashMap::new();

    while let Ok(info) = frame::Info::from_data_and_index(data, index) {
        if info.body_size > 0 {
            let mut body = Cursor::new(&data[info.body_range.clone()]);
            let mut did_ping_update = false;

            while body
                .read_message_type()
                .is_ok_and(|t| t == MessageType::UpdatePing)
            {
                if let Ok(u) = body.read_update_ping() {
                    let pings = total_pings.entry(u.player_number).or_default();
                    pings.push(u.ping);

                    body.set_position(body.position() + 2); // skip UpdatePl
                    did_ping_update = true;
                }
            }

            if did_ping_update {
                sample_count += 1;
            }
        }

        if sample_count >= max_samples {
            break;
        }

        index += info.size;
    }

    if total_pings.is_empty() {
        return Ok(HashMap::new());
    }

    let mut average_ping: HashMap<u8, u32> = HashMap::new();

    for (pnum, pings) in total_pings.iter() {
        let pings_sum = pings.iter().sum::<u16>();
        let avg_ping = (pings_sum as f32 / pings.len() as f32) as u32;
        average_ping.insert(*pnum, avg_ping);
    }

    Ok(average_ping)
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_pings() -> Result<()> {
        let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        let pings = pings(&demo_data)?;

        assert_eq!(pings.get(&0), Some(&26));
        assert_eq!(pings.get(&1), Some(&26));
        assert_eq!(pings.get(&2), Some(&666));
        assert_eq!(pings.get(&3), Some(&12));
        assert_eq!(pings.get(&4), Some(&28));
        assert_eq!(pings.get(&5), Some(&12));
        assert_eq!(pings.get(&6), Some(&12));
        assert_eq!(pings.get(&7), Some(&12));
        assert_eq!(pings.get(&8), Some(&12));

        Ok(())
    }
}
