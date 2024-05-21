use std::collections::HashMap;
use std::io::Cursor;

use anyhow::{anyhow as e, Result};

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
        return Err(e!("Unable to read pings"));
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
        {
            let demo_data: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let err = pings(&demo_data).unwrap_err();
            assert_eq!(err.to_string(), "Unable to read pings".to_string());
        }

        {
            let demo_data = read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd")?;
            let pings = pings(&demo_data)?;
            assert_eq!(pings.get(&0), Some(&25));
            assert_eq!(pings.get(&1), Some(&620));
            assert_eq!(pings.get(&2), Some(&29));
        }

        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd");
            let pings = pings(&demo_data?)?;
            assert_eq!(pings.get(&0), Some(&26));
            assert_eq!(pings.get(&1), Some(&26));
            assert_eq!(pings.get(&2), Some(&666));
            assert_eq!(pings.get(&3), Some(&12));
            assert_eq!(pings.get(&4), Some(&28));
            assert_eq!(pings.get(&5), Some(&12));
            assert_eq!(pings.get(&6), Some(&12));
            assert_eq!(pings.get(&7), Some(&12));
            assert_eq!(pings.get(&8), Some(&12));
        }

        {
            let demo_data = read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd");
            let pings = pings(&demo_data?)?;
            assert_eq!(pings.get(&0), Some(&19));
            assert_eq!(pings.get(&1), Some(&12));
            assert_eq!(pings.get(&2), Some(&30));
            assert_eq!(pings.get(&3), Some(&12));
            assert_eq!(pings.get(&4), Some(&13));
            assert_eq!(pings.get(&5), Some(&50));
            assert_eq!(pings.get(&6), Some(&39));
            assert_eq!(pings.get(&7), Some(&12));
        }

        {
            let demo_data = read("tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd");
            let pings = pings(&demo_data?)?;
            assert_eq!(pings.get(&0), Some(&14));
            assert_eq!(pings.get(&1), Some(&52));
            assert_eq!(pings.get(&2), Some(&38));
            assert_eq!(pings.get(&3), Some(&12));
            assert_eq!(pings.get(&4), Some(&666));
            assert_eq!(pings.get(&5), Some(&25));
        }

        {
            let demo_data = read("tests/files/ffa_5[dm4]20240501-1229.mvd")?;
            let pings = pings(&demo_data)?;
            assert_eq!(pings.get(&0), Some(&557));
            assert_eq!(pings.get(&1), Some(&12));
            assert_eq!(pings.get(&2), Some(&12));
            assert_eq!(pings.get(&3), Some(&10));
            assert_eq!(pings.get(&4), Some(&50));
            assert_eq!(pings.get(&5), Some(&10));
            assert_eq!(pings.get(&6), Some(&10));
            assert_eq!(pings.get(&7), Some(&10));
        }

        Ok(())
    }
}
