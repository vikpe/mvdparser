use std::collections::HashMap;
use std::io::Cursor;

use anyhow::{anyhow as e, Result};

use crate::mvd::message::io::ReadMessages;
use crate::mvd::message::print::ReadPrint;
use crate::mvd::message::update_frags::ReadUpdateFrags;
use crate::mvd::message::Print;
use crate::qw::{MessageType, PrintId};
use crate::{clients, fragevent, frame};

pub fn frags(data: &[u8]) -> HashMap<String, i32> {
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
                if p.id == PrintId::Medium && !p.content.is_empty() {
                    print_frames.push((p, frame_info.clone()))
                }
            }
        }

        index += frame_info.size;
    }

    let mut frags: HashMap<String, i32> = HashMap::new();

    for (print, frame_info) in print_frames {
        let content_u = quake_text::bytestr::to_unicode(&print.content);

        match fragevent::FragEvent::try_from(content_u.trim_end()) {
            Ok(event) => {
                //println!("{:?}", event);
                match event {
                    fragevent::FragEvent::Frag { killer, .. } => {
                        let killer = frags.entry(killer).or_insert(0);
                        *killer += 1;
                    }
                    fragevent::FragEvent::Death { player } => {
                        let player = frags.entry(player).or_insert(0);
                        *player -= 1;
                    }
                    fragevent::FragEvent::Suicide { player } => {
                        let player = frags.entry(player).or_insert(0);
                        *player -= 2;
                    }
                    fragevent::FragEvent::SuicideByWeapon { player } => {
                        let player = frags.entry(player).or_insert(0);
                        *player -= 1;
                    }
                    fragevent::FragEvent::Teamkill { killer } => {
                        let killer = frags.entry(killer).or_insert(0);
                        *killer -= 1;
                    }
                    fragevent::FragEvent::TeamkillByUnknown { victim } => {
                        if let Ok(name) = find_team_killer(data, frame_info.index, &victim) {
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

    frags
}

fn find_team_killer(data: &[u8], index: usize, victim_name: &str) -> Result<String> {
    let mut index = index;
    let mut frame_count: usize = 1;
    let mut frag_update_player_numbers: Vec<u8> = vec![];
    const MAX_FRAME_COUNT: usize = 4;

    while let Ok(info) = frame::Info::from_data_and_index(data, index) {
        if frame_count >= MAX_FRAME_COUNT {
            break;
        } else if info.body_size == 0 {
            index += info.size;
            continue;
        }

        let mut body = Cursor::new(&data[info.body_range.clone()]);

        if frame_count == 1 {
            body.read_print().ok();
        }

        if body
            .read_message_type()
            .is_ok_and(|t| t == MessageType::UpdateFrags)
        {
            if let Ok(u) = body.read_update_frags() {
                frag_update_player_numbers.push(u.player_number);
            }
        }

        index += info.size;
        frame_count += 1;
    }

    if frag_update_player_numbers.is_empty() {
        return Err(e!("Unable to find nearby frag updates"));
    }

    let clients = clients::clients(data)?;
    let Some(victim_client) = clients.iter().find(|c| c.name == *victim_name) else {
        return Err(e!("Unable to find victim"));
    };

    let killer = frag_update_player_numbers
        .iter()
        .filter_map(|player_number| clients.iter().find(|c| c.number == *player_number))
        .find(|c| c.team == victim_client.team && c.name != victim_client.name)
        .map(|c| c.name.clone());

    match killer {
        Some(killer) => Ok(killer),
        None => Err(e!("Unable to find team killer")),
    }
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
            let demo_data = read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd")?;
            let frags_map = frags(&demo_data);
            assert_eq!(frags_map.len(), 2);
            assert_eq!(frags_map.get("eQu"), Some(&19));
            assert_eq!(frags_map.get("KabÏÏm"), Some(&20));
        }

        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            let frags_map = frags(&demo_data);
            assert_eq!(frags_map.len(), 2);
            assert_eq!(frags_map.get("HoLy"), Some(&25));
            assert_eq!(frags_map.get("äáçï"), Some(&31));
        }

        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            let frags_map = frags(&demo_data);
            assert_eq!(frags_map.len(), 8);
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
            let demo_data = read("tests/files/ffa_5[dm4]20240501-1229.mvd")?;
            let frags_map = frags(&demo_data);
            assert_eq!(frags_map.len(), 5);
            assert_eq!(frags_map.get("test"), Some(&4));
            assert_eq!(frags_map.get("/ bro"), Some(&6));
            assert_eq!(frags_map.get("/ goldenboy"), Some(&5));
            assert_eq!(frags_map.get("/ tincan"), Some(&8));
            assert_eq!(frags_map.get("/ grue"), Some(&6));
        }

        Ok(())
    }
}
