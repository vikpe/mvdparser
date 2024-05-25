use std::collections::HashMap;
use std::io::Cursor;

use bstr::ByteVec;

use crate::flagevent::FlagEvent;
use crate::flagprint::{X_CAPTURED_FLAG, X_GOT_FLAG, X_RETURNED_FLAG};
use crate::frame;
use crate::mvd::message::io::ReadMessages;
use crate::mvd::message::print::ReadPrint;
use crate::qw::MessageType;

const MSG_NO_WEAPON: &[u8; 10] = b"no weapon\n";
const MSG_NOT_ENOUGH_AMMO: &[u8; 16] = b"not enough ammo\n";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PlayerFlagEvents {
    capture_flag: u8,
    got_flag: u8,
    return_flag: u8,
    return_flag_assist: u8,
    defend_flag: u8,
    defend_flag_carrier: u8,
    defend_flag_carrier_vs_aggressive: u8,
}

pub fn is_message_suffix(message: &str) -> bool {
    [
        X_RETURNED_FLAG.to_vec(),
        X_GOT_FLAG.to_vec(),
        X_CAPTURED_FLAG.to_vec(),
    ]
    .iter()
    .flatten()
    .any(|n| message.trim_end().ends_with(n))
}

pub fn player_flag_events(data: &[u8]) -> HashMap<String, PlayerFlagEvents> {
    let mut index = 0;

    let mut prints: Vec<Vec<u8>> = vec![];

    while let Ok(frame_info) = frame::Info::from_data_and_index(data, index) {
        if frame_info.body_size == 0 {
            index += frame_info.size;
            continue;
        }

        let mut body = Cursor::new(&data[frame_info.clone().body_range]);
        let mut current_print: Vec<u8> = vec![];

        while body
            .read_message_type()
            .is_ok_and(|t| t == MessageType::Print)
        {
            if let Ok(p) = body.read_print() {
                if !p.content.is_empty()
                    && p.content != MSG_NO_WEAPON
                    && p.content != MSG_NOT_ENOUGH_AMMO
                {
                    let content_utf8 = quake_text::bytestr::to_utf8(&p.content);

                    if content_utf8.contains(" capture ") {
                        println!("{:?}", content_utf8.trim_end());
                    }

                    if current_print.is_empty() {
                        current_print.push_str(&p.content);
                    } else {
                        if is_message_suffix(&content_utf8) {
                            current_print.push_str(&p.content);
                        }

                        prints.push(current_print.clone());
                        current_print = vec![];
                    }
                }
            }
        }

        if !current_print.is_empty() {
            prints.push(current_print);
        }

        index += frame_info.size;
    }

    let mut player_flag_events: HashMap<String, PlayerFlagEvents> = HashMap::new();

    for print in prints {
        match FlagEvent::try_from(&print[..]) {
            Ok(event) => match event {
                FlagEvent::CapturedFlag { player } => {
                    let pfe = player_flag_events.entry(player).or_default();
                    pfe.capture_flag += 1;
                }
                FlagEvent::DefendsFlag { player } => {
                    let pfe = player_flag_events.entry(player).or_default();
                    pfe.defend_flag += 1;
                }
                FlagEvent::DefendsFlagCarrier { player } => {
                    let pfe = player_flag_events.entry(player).or_default();
                    pfe.defend_flag_carrier += 1;
                }
                FlagEvent::DefendFlagCarrierVsAggressive { player } => {
                    let pfe = player_flag_events.entry(player).or_default();
                    pfe.defend_flag_carrier_vs_aggressive += 1;
                }
                FlagEvent::GotFlag { player } => {
                    let pfe = player_flag_events.entry(player).or_default();
                    pfe.got_flag += 1;
                }
                FlagEvent::ReturnedFlag { player } => {
                    let pfe = player_flag_events.entry(player).or_default();
                    pfe.return_flag += 1;
                }
                FlagEvent::ReturnedFlagAssist { player } => {
                    let pfe = player_flag_events.entry(player).or_default();
                    pfe.return_flag_assist += 1;
                }
            },
            Err(_e) => {
                // println!("UNKNOWN {:?}", e);
            }
        }
    }

    player_flag_events
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_player_flag_events() -> Result<()> {
        {
            let demo_data = read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?;
            let events = player_flag_events(&demo_data);
            for (player, pfe) in events.iter() {
                println!("{:?} {:?}", player, pfe);
            }
            // assert_eq!(events.len(), 10);

            assert_eq!(
                events.get("ì÷ú\u{AD}velocity"),
                Some(&PlayerFlagEvents {
                    capture_flag: 6,
                    got_flag: 7,
                    return_flag: 4,
                    return_flag_assist: 1,
                    defend_flag: 4,
                    defend_flag_carrier: 3,
                    defend_flag_carrier_vs_aggressive: 0,
                })
            );
            /*assert_eq!(events.get("ì÷ú\u{AD}lethalwiz"), Some(&140));
            assert_eq!(events.get("ì÷ú\u{AD}xunito"), Some(&128));
            assert_eq!(events.get("lwz-brunelson"), Some(&120));
            assert_eq!(events.get("ì÷ú\u{AD}lag"), Some(&118));
            assert_eq!(events.get("CCTãáöåòïî"), Some(&29));
            assert_eq!(events.get("CCTâéìì"), Some(&23));
            assert_eq!(events.get("CCTÓèéîéîç"), Some(&19));
            assert_eq!(events.get("CCTäêåöõìóë"), Some(&15));
            assert_eq!(events.get("CCTÈåíìïãë"), Some(&10));*/
        }

        Ok(())
    }
}
