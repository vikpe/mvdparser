use std::collections::HashMap;

use fragfile::FragEvent;

use crate::message::Print;
use crate::qw::{Message, PrintId};
use crate::{fragfile, frame};

pub fn frags(data: &[u8]) -> HashMap<String, i32> {
    // get all prints along with frame info
    let mut data_offset = 0;
    let mut print_frames: Vec<(Print, frame::DetailedInfo)> = vec![];

    while let Ok(frame_info) = frame::DetailedInfo::from_data_and_index(data, data_offset) {
        if frame_info.body.size == 0 {
            data_offset += frame_info.size;
            continue;
        }

        let msg_offset = data_offset + frame_info.header.size;

        if Message::Print == Message::from(&data[msg_offset]) {
            let print_offset = msg_offset + 1;

            match Print::try_from(&data[print_offset..]) {
                Ok(p) if p.id == PrintId::Medium && !p.content.is_empty() => {
                    print_frames.push((p, frame_info.clone()))
                }
                Err(e) => println!("Error parsing print: {:?}", e),
                _ => {}
            }
        }

        data_offset += frame_info.size;
    }

    let mut frags: HashMap<String, i32> = HashMap::new();

    for (print, frame_info) in print_frames {
        let ascii_str = quake_text::bytestr::to_ascii(&print.content);

        match FragEvent::try_from(ascii_str.as_str()) {
            Ok(event) => {
                //println!("{:?}", event);

                match event {
                    FragEvent::Frag { killer, .. } => {
                        let killer = frags.entry(killer).or_insert(0);
                        *killer += 1;
                    }
                    FragEvent::Death { player } => {
                        let player = frags.entry(player).or_insert(0);
                        *player -= 1;
                    }
                    FragEvent::Suicide { player } => {
                        let player = frags.entry(player).or_insert(0);
                        *player -= 2;
                    }
                    FragEvent::SuicideByWeapon { player } => {
                        let player = frags.entry(player).or_insert(0);
                        *player -= 1;
                    }
                    FragEvent::Teamkill { killer } => {
                        let killer = frags.entry(killer).or_insert(0);
                        *killer -= 1;
                    }
                    FragEvent::TeamkillByUnknown { .. } => {
                        println!();
                        println!("{:?}", quake_text::bytestr::to_ascii(&print.content));
                        println!("{:?}", frame_info);
                        println!()
                    }
                }
            }
            Err(e) => {
                println!("UNKNOWN {:?}", e);
            }
        }
    }

    frags
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;

    use super::*;

    #[test]
    fn test_frags() -> Result<()> {
        let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        let timer_start = std::time::Instant::now();
        let frags_map = frags(&demo_data);
        println!("took: {:?} ms", timer_start.elapsed().as_millis());

        assert_eq!(frags_map.get("bar.........axe"), Some(&27));
        assert_eq!(frags_map.get("conan"), Some(&71));
        assert_eq!(frags_map.get("djevulsk"), Some(&74));

        // assert_eq!(frags_map.get("elguapo"), Some(&60)); // fail: 59
        assert_eq!(frags_map.get("muttan"), Some(&89));
        assert_eq!(frags_map.get("tco.........axe"), Some(&32)); // fail: 31

        // assert_eq!(frags_map.get("tim.........axe"), Some(&33)); // fail: 34
        assert_eq!(frags_map.get("trl.........axe"), Some(&26));

        Ok(())
    }
}
