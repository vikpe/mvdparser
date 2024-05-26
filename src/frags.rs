use std::collections::HashMap;
use std::io::Cursor;

use anyhow::{anyhow as e, Result};

use fragevent::FragEvent;

use crate::clients::player_clients;
use crate::mvd::message::io::ReadMessages;
use crate::mvd::message::print::ReadPrint;
use crate::mvd::message::update_frags::ReadUpdateFrags;
use crate::mvd::message::Print;
use crate::qw::{MessageType, PrintId};
use crate::{clients, ctf, fragevent, frame, ktxstats_v3, serverinfo};

pub fn frags(data: &[u8]) -> Result<HashMap<String, i32>> {
    frags_from_ktxstats(data).or_else(|_| frags_from_parsing(data))
}

fn frags_from_ktxstats(data: &[u8]) -> Result<HashMap<String, i32>> {
    let stats = ktxstats_v3(data)?;
    let frags_per_player = stats
        .players
        .iter()
        .map(|p| (p.name.clone(), p.stats.frags));
    Ok(HashMap::from_iter(frags_per_player))
}

fn frags_from_parsing(data: &[u8]) -> Result<HashMap<String, i32>> {
    let mut index = 0;
    let mut print_frames: Vec<(Print, frame::Info)> = vec![];

    while let Ok(frame_info) = frame::Info::from_data_and_index(data, index) {
        if frame_info.body_size == 0 {
            index += frame_info.size;
            continue;
        }

        let mut body = Cursor::new(&data[frame_info.clone().body_range]);

        while body
            .read_message_type()
            .is_ok_and(|t| t == MessageType::Print)
        {
            if let Ok(p) = body.read_print() {
                if !p.content.is_empty() && PrintId::Medium == p.id {
                    print_frames.push((p, frame_info.clone()))
                }
            }
        }

        index += frame_info.size;
    }

    let players = player_clients(data)?;
    let mut frags_pp = HashMap::from_iter(players.iter().cloned().map(|c| (c.name.clone(), 0)));

    for (print, frame_info) in print_frames {
        let print_u = quake_text::bytestr::to_unicode(&print.content);

        match FragEvent::try_from(print_u.trim_end()) {
            Ok(event) => match event {
                FragEvent::Frag { killer, .. } => {
                    let killer = frags_pp.entry(killer).or_insert(0);
                    *killer += 1;
                }
                FragEvent::Death { player } => {
                    let player = frags_pp.entry(player).or_insert(0);
                    *player -= 1;
                }
                FragEvent::Suicide { player } => {
                    let player = frags_pp.entry(player).or_insert(0);
                    *player -= 2;
                }
                FragEvent::SuicideByWeapon { player } => {
                    let player = frags_pp.entry(player).or_insert(0);
                    *player -= 1;
                }
                FragEvent::Teamkill { killer } => {
                    let killer = frags_pp.entry(killer).or_insert(0);
                    *killer -= 1;
                }
                FragEvent::TeamkillByUnknown { victim } => {
                    if let Ok(name) = find_team_killer(data, frame_info.index, &victim) {
                        let killer = frags_pp.entry(name).or_insert(0);
                        *killer -= 1;
                    }
                }
            },
            Err(_e) => {
                // println!("UNKNOWN {:?}", e);
            }
        }
    }

    // if ctf, add points
    if serverinfo(data).is_some_and(|i| serverinfo::analyze::is_ctf(&i)) {
        let points_pp = ctf::points(data)?;

        for (name, points) in points_pp {
            if let Some(player_frags) = frags_pp.get_mut(&name) {
                *player_frags += points;
            }
        }
    }

    Ok(frags_pp)
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
            assert_eq!(
                frags_from_ktxstats(&demo_data)?,
                frags_from_parsing(&demo_data)?
            );
        }
        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            assert_eq!(
                frags_from_ktxstats(&demo_data)?,
                frags_from_parsing(&demo_data)?
            );
        }
        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            assert_eq!(
                frags_from_ktxstats(&demo_data)?,
                frags_from_parsing(&demo_data)?
            );
        }
        {
            let demo_data = read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?;
            assert_eq!(
                frags_from_ktxstats(&demo_data)?,
                frags_from_parsing(&demo_data)?
            );
        }
        {
            let demo_data = read("tests/files/ffa_5[dm4]20240501-1229.mvd")?;
            assert_eq!(
                frags_from_ktxstats(&demo_data)?,
                frags_from_parsing(&demo_data)?
            );
        }

        Ok(())
    }

    #[test]
    fn test_frags_from_ktxstats_v3() -> Result<()> {
        {
            let demo_data = read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd")?;
            let frags = frags_from_ktxstats(&demo_data)?;
            assert_eq!(frags.len(), 2);
            assert_eq!(frags.get("KabÏÏm"), Some(&20));
            assert_eq!(frags.get("eQu"), Some(&19));
        }

        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            let frags = frags_from_ktxstats(&demo_data)?;
            assert_eq!(frags.len(), 2);
            assert_eq!(frags.get("äáçï"), Some(&31));
            assert_eq!(frags.get("HoLy"), Some(&25));
        }

        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            let frags = frags_from_ktxstats(&demo_data)?;
            assert_eq!(frags.len(), 8);
            assert_eq!(frags.get("muttan"), Some(&89));
            assert_eq!(frags.get("djevulsk"), Some(&74));
            assert_eq!(frags.get("conan"), Some(&71));
            assert_eq!(frags.get("elguapo"), Some(&60));
            assert_eq!(frags.get("tim.........áøå"), Some(&33));
            assert_eq!(frags.get("tco.........áøå"), Some(&32));
            assert_eq!(frags.get("bar.........áøå"), Some(&27));
            assert_eq!(frags.get("trl.........áøå"), Some(&26));
        }

        {
            let demo_data = read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?;
            let frags = frags_from_ktxstats(&demo_data)?;
            assert_eq!(frags.len(), 10);
            assert_eq!(frags.get("ì÷ú\u{AD}velocity"), Some(&164));
            assert_eq!(frags.get("ì÷ú\u{AD}lethalwiz"), Some(&140));
            assert_eq!(frags.get("ì÷ú\u{AD}xunito"), Some(&128));
            assert_eq!(frags.get("lwz-brunelson"), Some(&120));
            assert_eq!(frags.get("ì÷ú\u{AD}lag"), Some(&118));
            assert_eq!(frags.get("CCTãáöåòïî"), Some(&29));
            assert_eq!(frags.get("CCTâéìì"), Some(&23));
            assert_eq!(frags.get("CCTÓèéîéîç"), Some(&19));
            assert_eq!(frags.get("CCTäêåöõìóë"), Some(&15));
            assert_eq!(frags.get("CCTÈåíìïãë"), Some(&10));
        }

        {
            let demo_data = read("tests/files/ffa_5[dm4]20240501-1229.mvd")?;
            let frags = frags_from_ktxstats(&demo_data)?;
            assert_eq!(frags.len(), 5);
            assert_eq!(frags.get("/ tincan"), Some(&8));
            assert_eq!(frags.get("/ bro"), Some(&6));
            assert_eq!(frags.get("/ grue"), Some(&6));
            assert_eq!(frags.get("/ goldenboy"), Some(&5));
            assert_eq!(frags.get("test"), Some(&4));
        }

        Ok(())
    }

    #[test]
    fn test_frags_from_parsing() -> Result<()> {
        {
            let demo_data = read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd")?;
            let frags = frags_from_parsing(&demo_data)?;
            assert_eq!(frags.len(), 2);
            assert_eq!(frags.get("KabÏÏm"), Some(&20));
            assert_eq!(frags.get("eQu"), Some(&19));
        }

        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            let frags = frags_from_parsing(&demo_data)?;
            assert_eq!(frags.len(), 2);
            assert_eq!(frags.get("äáçï"), Some(&31));
            assert_eq!(frags.get("HoLy"), Some(&25));
        }

        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            let frags = frags_from_parsing(&demo_data)?;
            assert_eq!(frags.len(), 8);
            assert_eq!(frags.get("muttan"), Some(&89));
            assert_eq!(frags.get("djevulsk"), Some(&74));
            assert_eq!(frags.get("conan"), Some(&71));
            assert_eq!(frags.get("elguapo"), Some(&60));
            assert_eq!(frags.get("tim.........áøå"), Some(&33));
            assert_eq!(frags.get("tco.........áøå"), Some(&32));
            assert_eq!(frags.get("bar.........áøå"), Some(&27));
            assert_eq!(frags.get("trl.........áøå"), Some(&26));
        }

        {
            let demo_data = read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?;
            let frags = frags_from_parsing(&demo_data)?;

            let mut players: Vec<String> = vec![];

            for (k, v) in &frags {
                let line = format!(
                    "{:0>3}\t{}",
                    v,
                    quake_text::unicode::to_utf8(k).to_lowercase()
                );
                players.push(line);
            }
            players.sort();
            players.reverse();

            for p in players {
                println!("{}", p);
            }

            assert_eq!(frags.len(), 10);
            assert_eq!(frags.get("ì÷ú\u{AD}velocity"), Some(&164));
            assert_eq!(frags.get("ì÷ú\u{AD}lethalwiz"), Some(&140));
            assert_eq!(frags.get("ì÷ú\u{AD}xunito"), Some(&128));
            assert_eq!(frags.get("lwz-brunelson"), Some(&120));
            assert_eq!(frags.get("ì÷ú\u{AD}lag"), Some(&118));
            assert_eq!(frags.get("CCTãáöåòïî"), Some(&29));
            assert_eq!(frags.get("CCTâéìì"), Some(&23));
            assert_eq!(frags.get("CCTÓèéîéîç"), Some(&19));
            assert_eq!(frags.get("CCTäêåöõìóë"), Some(&15));
            assert_eq!(frags.get("CCTÈåíìïãë"), Some(&10));
        }

        {
            let demo_data = read("tests/files/ffa_5[dm4]20240501-1229.mvd")?;
            let frags = frags_from_parsing(&demo_data)?;
            assert_eq!(frags.len(), 5);
            assert_eq!(frags.get("/ tincan"), Some(&8));
            assert_eq!(frags.get("/ bro"), Some(&6));
            assert_eq!(frags.get("/ grue"), Some(&6));
            assert_eq!(frags.get("/ goldenboy"), Some(&5));
            assert_eq!(frags.get("test"), Some(&4));
        }

        Ok(())
    }
}
