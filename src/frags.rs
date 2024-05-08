use std::collections::HashMap;

use bstr::ByteSlice;
use quake_clientinfo::Clientinfo;

use fragfile::FragEvent;

use crate::message::Print;
use crate::qw::{Message, PrintId};
use crate::{clientinfo, fragfile, frame};

pub fn frags(data: &[u8]) -> HashMap<String, i32> {
    let mut index = 0;
    let mut print_frames: Vec<(Print, frame::Info)> = vec![];

    while let Ok(frame_info) = frame::Info::from_data_and_index(data, index) {
        if frame_info.body_size == 0 {
            index += frame_info.size;
            continue;
        }

        let msg_offset = frame_info.header_range.end;

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

        index += frame_info.size;
    }

    // let clientinfo = clientinfo::clientinfo(data).unwrap();

    /*for (index, p) in clientinfo.iter().enumerate() {
        println!("{}: {:?}", index, p.name);
    }*/

    let mut frags: HashMap<String, i32> = HashMap::new();

    for (print, frame_info) in print_frames {
        let content_u = quake_text::bytestr::to_unicode(&print.content);

        match FragEvent::try_from(content_u.as_str()) {
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
                    FragEvent::TeamkillByUnknown { victim } => {
                        if let Some(name) = find_team_killer(data, frame_info.index, &victim) {
                            // println!("KILLER: {:?} -> {}", content_u, name);
                            let killer = frags.entry(name).or_insert(0);
                            *killer -= 1;
                        }
                    }
                }
            }
            Err(e) => {
                println!("UNKNOWN {:?}", e);
            }
        }
    }

    // println!("FRAGS: {:?}", frags);

    frags
}

fn find_team_killer(data: &[u8], index: usize, victim_name: &str) -> Option<String> {
    let mut index = index;
    let mut frame_count: usize = 0;
    let mut frag_update_player_numbers: Vec<usize> = vec![];
    let max_frame_count = 3;

    while let Ok(frame_info) = frame::Info::from_data_and_index(data, index) {
        if frame_count >= max_frame_count {
            break;
        }

        if frame_info.body_size == 0 {
            index += frame_info.size;
            continue;
        }

        let msg_offset = frame_info.header_range.end;
        let msg = Message::from(&data[msg_offset]);

        if Message::Print == msg {
            let next_msg_byte = data[frame_info.body_range.clone()]
                .find([0])
                .map(|n| frame_info.body_range.start + n + 1)?;

            if Message::UpdateFrags == Message::from(&data[next_msg_byte]) {
                frag_update_player_numbers.push(data[next_msg_byte + 1] as usize);
            }
        } else if Message::UpdateFrags == msg {
            frag_update_player_numbers.push(data[msg_offset + 1] as usize);
        }

        index += frame_info.size;
        frame_count += 1;
    }

    if frag_update_player_numbers.is_empty() {
        return None;
    }

    // println!("nearby frag updates");
    // println!("{:?}", frag_update_player_numbers);
    // println!("victim: {:?}", victim_name);

    let clients = clientinfo::clientinfo(data)?;
    let victim_client = clients.iter().find(|c| {
        // println!("{} vs {:?}", c.name.clone().unwrap(), victim_name);
        c.name.clone().unwrap() == *victim_name
    })?;

    // println!("victim client: {:?}", victim_client.name);

    let clientm: HashMap<usize, &Clientinfo> = HashMap::from_iter(clients.iter().enumerate());
    // println!("client map: {:?}", clientm);

    frag_update_player_numbers
        .iter()
        .filter_map(|p| clientm.get(p))
        .find(|client| {
            client.team.clone().unwrap() == victim_client.team.clone().unwrap()
                && client.name.clone().unwrap() != victim_client.name.clone().unwrap()
        })
        .map(|c| c.name.clone().unwrap())
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_frags() -> Result<()> {
        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            let frags_map = frags(&demo_data);
            assert_eq!(frags_map.get("conan"), Some(&71));
            assert_eq!(frags_map.get("djevulsk"), Some(&74));
            assert_eq!(frags_map.get("elguapo"), Some(&60));
            assert_eq!(frags_map.get("muttan"), Some(&89));
            assert_eq!(frags_map.get("tco.........áøå"), Some(&32));
            assert_eq!(frags_map.get("trl.........áøå"), Some(&26));
            assert_eq!(frags_map.get("tim.........áøå"), Some(&33));
            assert_eq!(frags_map.get("bar.........áøå"), Some(&27));
        }

        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            let frags_map = frags(&demo_data);
            assert_eq!(frags_map.get("HoLy"), Some(&25));
            assert_eq!(frags_map.get("äáçï"), Some(&31));
        }

        {
            let demo_data = read("tests/files/ffa_5[dm4]20240501-1229.mvd")?;
            let frags_map = frags(&demo_data);
            assert_eq!(frags_map.get("test"), Some(&4));
            assert_eq!(frags_map.get("/ bro"), Some(&6));
            assert_eq!(frags_map.get("/ goldenboy"), Some(&5));
            assert_eq!(frags_map.get("/ tincan"), Some(&8));
            assert_eq!(frags_map.get("/ grue"), Some(&6));
        }

        Ok(())
    }
}
