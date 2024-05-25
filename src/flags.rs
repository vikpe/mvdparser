use std::collections::HashMap;
use std::io::Cursor;

use anyhow::{anyhow as e, Result};

use fragfile::FragEvent;

use crate::mvd::message::io::ReadMessages;
use crate::mvd::message::print::ReadPrint;
use crate::mvd::message::update_frags::ReadUpdateFrags;
use crate::mvd::message::Print;
use crate::qw::{MessageType, PrintId};
use crate::{clients, fragfile, frame};

const NO_WEAPON: &[u8; 10] = b"no weapon\n";
const NOT_ENOUGH_AMMO: &[u8; 16] = b"not enough ammo\n";

pub fn flags(data: &[u8]) -> HashMap<String, i32> {
    let mut index = 0;
    let mut print_frames: Vec<(Print, frame::Info)> = vec![];

    while let Ok(frame_info) = frame::Info::from_data_and_index(data, index) {
        if frame_info.body_size == 0 {
            index += frame_info.size;
            continue;
        }

        let mut body = Cursor::new(&data[frame_info.clone().body_range]);

        if body
            .read_message_type()
            .is_ok_and(|t| t == MessageType::Print)
        {
            if let Ok(p) = body.read_print() {
                if [PrintId::Medium, PrintId::High].contains(&p.id)
                    && !p.content.is_empty()
                    && p.content != NO_WEAPON
                    && p.content != NOT_ENOUGH_AMMO
                {
                    let utf8val = quake_text::bytestr::to_utf8(&p.content);

                    if utf8val.contains("took") {
                        println!("{}", utf8val);
                    }

                    print_frames.push((p, frame_info.clone()))
                }
            }
        }

        index += frame_info.size;
    }

    let mut frags: HashMap<String, i32> = HashMap::new();

    for (print, frame_info) in print_frames {
        let content_u = quake_text::bytestr::to_unicode(&print.content);

        match FragEvent::try_from(content_u.trim_end()) {
            Ok(event) => match event {
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
                FragEvent::TeamkillByUnknown { victim } => {
                    if let Ok(name) = find_team_killer(data, frame_info.index, &victim) {
                        let killer = frags.entry(name).or_insert(0);
                        *killer -= 1;
                    }
                }
                FragEvent::FlagAlert { player, event } => {
                    println!("FLAG ALERT: {:?} -> {:?}", player, event);
                }
            },
            Err(e) => {
                // println!("UNKNOWN {:?}", e);
            }
        }
    }

    frags
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_flagevents() -> Result<()> {
        {
            let demo_data = read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?;
            let frags_map = flags(&demo_data);
            assert_eq!(frags_map.len(), 11);
            assert_eq!(frags_map.get("ì÷ú\u{AD}velocity"), Some(&164));
            assert_eq!(frags_map.get("ì÷ú\u{AD}lethalwiz"), Some(&140));
            assert_eq!(frags_map.get("ì÷ú\u{AD}xunito"), Some(&128));
            assert_eq!(frags_map.get("lwz-brunelson"), Some(&120));
            assert_eq!(frags_map.get("ì÷ú\u{AD}lag"), Some(&118));
            assert_eq!(frags_map.get("CCTãáöåòïî"), Some(&29));
            assert_eq!(frags_map.get("CCTâéìì"), Some(&23));
            assert_eq!(frags_map.get("CCTÓèéîéîç"), Some(&19));
            assert_eq!(frags_map.get("CCTäêåöõìóë"), Some(&15));
            assert_eq!(frags_map.get("CCTÈåíìïãë"), Some(&10));
        }

        Ok(())
    }
}
