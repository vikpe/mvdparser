use std::collections::HashMap;
use std::io::Cursor;

use anyhow::Result;
use bstr::ByteSlice;

use crate::clients::player_clients;
use crate::flagevent::FlagEvent;
use crate::flagprint;
use crate::frame;
use crate::mvd::message::io::ReadMessages;
use crate::mvd::message::print::ReadPrint;
use crate::qw::{MessageType, PrintId};

pub mod bonus {
    pub const CAPTURE: i32 = 15;
    pub const CAPTURE_TEAM: i32 = 10;
    pub const CARRIER_DEFEND: i32 = 1;
    pub const CARRIER_DEFEND_VS_AGGRESSIVE: i32 = 2;
    pub const CARRIER_FRAG: i32 = 2;
    pub const FLAG_DEFEND: i32 = 2;
    pub const RETURN_FLAG_ASSIST: i32 = 1;
    pub const RETURN_FLAG: i32 = 1;
}

pub fn player_flag_events(data: &[u8]) -> Result<HashMap<String, PlayerFlagEvents>> {
    let mut index = 0;

    let mut prints: Vec<Vec<u8>> = vec![];

    while let Ok(frame_info) = frame::Info::from_data_and_index(data, index) {
        if frame_info.body_size == 0 || data[frame_info.clone().body_range].find(b"flag").is_none()
        {
            index += frame_info.size;
            continue;
        }

        let mut body = Cursor::new(&data[frame_info.clone().body_range]);
        let mut current_print: Vec<u8> = vec![];

        while let Ok(mt) = body.read_message_type() {
            if mt == MessageType::Print {
                if let Ok(p) = body.read_print() {
                    if p.id == PrintId::High && !p.content.is_empty() {
                        if current_print.is_empty() {
                            current_print.extend_from_slice(&p.content);
                        } else {
                            if is_message_suffix(&p.content) {
                                current_print.extend_from_slice(&p.content);
                            }

                            prints.push(current_print.clone());
                            current_print = vec![];
                        }
                    }
                }
            }
        }

        if !current_print.is_empty() {
            prints.push(current_print);
        }

        index += frame_info.size;
    }

    let players = player_clients(data)?;
    let mut events_pp = HashMap::from_iter(
        players
            .iter()
            .cloned()
            .map(|c| (c.name.clone(), PlayerFlagEvents::default())),
    );

    for print in prints {
        let print_u = quake_text::bytestr::to_unicode(&print);

        match FlagEvent::try_from(print_u.as_str()) {
            Ok(event) => match event {
                FlagEvent::CapturedFlag { player } => {
                    let pfe = events_pp.entry(player).or_default();
                    pfe.captures += 1;
                }
                FlagEvent::DefendsFlag { player } => {
                    let pfe = events_pp.entry(player).or_default();
                    pfe.defends += 1;
                }
                FlagEvent::DefendsFlagCarrier { player } => {
                    let pfe = events_pp.entry(player).or_default();
                    pfe.carrier_defends += 1;
                }
                FlagEvent::DefendsFlagCarrierVsAggressive { player } => {
                    let pfe = events_pp.entry(player).or_default();
                    pfe.carrier_defends_vs_aggressive += 1;
                }
                FlagEvent::GotFlag { player } => {
                    let pfe = events_pp.entry(player).or_default();
                    pfe.pickups += 1;
                }
                FlagEvent::ReturnedFlag { player } => {
                    let pfe = events_pp.entry(player).or_default();
                    pfe.returns += 1;
                }
                FlagEvent::ReturnedFlagAssist { player } => {
                    let pfe = events_pp.entry(player).or_default();
                    pfe.carrier_frags += 1;
                }
            },
            Err(_e) => {
                // println!("UNKNOWN {:?}", e);
            }
        }
    }

    Ok(events_pp)
}

fn is_message_suffix(print: &[u8]) -> bool {
    [
        flagprint::X_RETURNED_FLAG.to_vec(),
        flagprint::X_GOT_FLAG.to_vec(),
        flagprint::X_CAPTURED_FLAG.to_vec(),
    ]
    .iter()
    .flatten()
    .any(|n| quake_text::bytestr::to_unicode(print).ends_with(n))
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PlayerFlagScores {
    pub player: i32,
    pub team: i32,
}

impl From<&PlayerFlagEvents> for PlayerFlagScores {
    fn from(value: &PlayerFlagEvents) -> Self {
        let mut scores = PlayerFlagScores::default();
        scores.player += bonus::CAPTURE * value.captures as i32;
        scores.player += bonus::CARRIER_FRAG * value.carrier_frags as i32;
        scores.player += bonus::CARRIER_DEFEND * value.carrier_defends as i32;
        scores.player +=
            bonus::CARRIER_DEFEND_VS_AGGRESSIVE * value.carrier_defends_vs_aggressive as i32;
        scores.player += bonus::FLAG_DEFEND * value.defends as i32;
        scores.player += bonus::RETURN_FLAG * value.returns as i32;
        scores.team += bonus::CAPTURE_TEAM * value.captures as i32;
        scores
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PlayerFlagEvents {
    captures: u8,
    pickups: u8,
    returns: u8,
    carrier_frags: u8,
    defends: u8,
    carrier_defends: u8,
    carrier_defends_vs_aggressive: u8,
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
            let events = player_flag_events(&demo_data)?;

            assert_eq!(events.len(), 10);

            assert_eq!(
                events.get("CCTãáöåòïî"),
                Some(&PlayerFlagEvents {
                    carrier_defends: 2,
                    carrier_defends_vs_aggressive: 2,
                    pickups: 2,
                    ..Default::default()
                })
            );
            assert_eq!(
                events.get("CCTâéìì"),
                Some(&PlayerFlagEvents {
                    carrier_defends: 1,
                    defends: 1,
                    returns: 2,
                    ..Default::default()
                })
            );
            assert_eq!(
                events.get("CCTÓèéîéîç"),
                Some(&PlayerFlagEvents {
                    pickups: 4,
                    returns: 2,
                    ..Default::default()
                })
            );
            assert_eq!(
                events.get("CCTäêåöõìóë"),
                Some(&PlayerFlagEvents {
                    pickups: 9,
                    ..Default::default()
                })
            );
            assert_eq!(
                events.get("CCTÈåíìïãë"),
                Some(&PlayerFlagEvents {
                    carrier_defends_vs_aggressive: 1,
                    pickups: 1,
                    returns: 1,
                    ..Default::default()
                })
            );

            assert_eq!(
                events.get("ì÷ú\u{AD}velocity"),
                Some(&PlayerFlagEvents {
                    captures: 6,
                    carrier_frags: 1,
                    defends: 4,
                    pickups: 7,
                    returns: 4,
                    ..Default::default()
                })
            );
            assert_eq!(
                events.get("ì÷ú\u{AD}lethalwiz"),
                Some(&PlayerFlagEvents {
                    captures: 2,
                    carrier_defends: 1,
                    defends: 3,
                    pickups: 3,
                    returns: 3,
                    ..Default::default()
                })
            );
            assert_eq!(
                events.get("ì÷ú\u{AD}lag"),
                Some(&PlayerFlagEvents {
                    captures: 1,
                    pickups: 4,
                    returns: 2,
                    ..Default::default()
                })
            );
            assert_eq!(
                events.get("ì÷ú\u{AD}xunito"),
                Some(&PlayerFlagEvents {
                    carrier_frags: 1,
                    defends: 3,
                    returns: 2,
                    ..Default::default()
                })
            );
            assert_eq!(
                events.get("lwz-brunelson"),
                Some(&PlayerFlagEvents {
                    carrier_defends: 1,
                    carrier_frags: 1,
                    defends: 1,
                    returns: 3,
                    ..Default::default()
                })
            );
        }

        Ok(())
    }

    #[test]
    fn test_player_flag_scores() {
        let events = PlayerFlagEvents {
            captures: 1,                      // 15
            pickups: 2,                       //
            returns: 3,                       // 3
            carrier_frags: 4,                 // 8
            defends: 5,                       // 10
            carrier_defends: 6,               // 6
            carrier_defends_vs_aggressive: 7, // 14
        };

        assert_eq!(
            PlayerFlagScores::from(&events),
            PlayerFlagScores {
                player: 56,
                team: 10
            }
        );
    }
}
