use anyhow::{anyhow as e, Result};

use crate::flagprint;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FlagEvent {
    CapturedFlag { player: String },
    GotFlag { player: String },
    DefendsFlag { player: String },
    DefendsFlagCarrier { player: String },
    DefendsFlagCarrierVsAggressive { player: String },
    ReturnedFlag { player: String },
    ReturnedFlagAssist { player: String },
}

impl TryFrom<&str> for FlagEvent {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let print_types: Vec<(usize, Vec<&str>)> = vec![
            (1, flagprint::X_CAPTURED_FLAG.to_vec()),
            (2, flagprint::X_DEFENDS_FLAG.to_vec()),
            (3, flagprint::X_DEFENDS_VS_AGGRESSIVE.to_vec()),
            (4, flagprint::X_DEFENDS_CARRIER.to_vec()),
            (5, flagprint::X_GOT_FLAG.to_vec()),
            (6, flagprint::X_RETURNED_FLAG.to_vec()),
            (7, flagprint::X_RETURNED_ASSIST.to_vec()),
        ];

        for (index, needles) in print_types {
            if let Some(pos) = needles.iter().find_map(|n| value.find(n)) {
                let player = value[..pos].to_string();

                match index {
                    1 => return Ok(FlagEvent::CapturedFlag { player }),
                    2 => return Ok(FlagEvent::DefendsFlag { player }),
                    3 => return Ok(FlagEvent::DefendsFlagCarrierVsAggressive { player }),
                    4 => return Ok(FlagEvent::DefendsFlagCarrier { player }),
                    5 => return Ok(FlagEvent::GotFlag { player }),
                    6 => return Ok(FlagEvent::ReturnedFlag { player }),
                    7 => return Ok(FlagEvent::ReturnedFlagAssist { player }),
                    _ => {}
                }
            }
        }

        Err(e!(r#"Unable to parse as flag event: "{}""#, value))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_flag_event() -> Result<()> {
        let test_cases: HashMap<&str, Result<FlagEvent>> = HashMap::from([
            (
                "FOO got the ÒÅÄ flag!\n",
                Ok(FlagEvent::GotFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "\u{10}FOO\u{11} got the ÒÅÄ flag!\n",
                Ok(FlagEvent::GotFlag {
                    player: "\u{10}FOO\u{11}".to_string(),
                }),
            ),
            (
                "FOO got the ÂÌÕÅ flag!\n",
                Ok(FlagEvent::GotFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO ãáðôõòåä the ÒÅÄ flag!\n",
                Ok(FlagEvent::CapturedFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO ãáðôõòåä the ÂÌÕÅ flag!\n",
                Ok(FlagEvent::CapturedFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO gets an assist for fragging the flag carrier!\n",
                Ok(FlagEvent::ReturnedFlagAssist {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO returned the ÒÅÄ flag!\n",
                Ok(FlagEvent::ReturnedFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO returned the ÂÌÕÅ flag!\n",
                Ok(FlagEvent::ReturnedFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends the ÒÅÄ flag\n",
                Ok(FlagEvent::DefendsFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends ÒÅÄ's flag carrier\n",
                Ok(FlagEvent::DefendsFlagCarrier {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends ÂÌÕÅ's flag carrier\n",
                Ok(FlagEvent::DefendsFlagCarrier {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends ÒÅÄ's flag carrier against an aggressive enemy\n",
                Ok(FlagEvent::DefendsFlagCarrierVsAggressive {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends ÂÌÕÅ's flag carrier against an aggressive enemy\n",
                Ok(FlagEvent::DefendsFlagCarrierVsAggressive {
                    player: "FOO".to_string(),
                }),
            ),
        ]);

        for (input, expected) in test_cases {
            let msg = format!(r#""{}" should equal {:?}"#, input, &expected);
            assert_eq!(FlagEvent::try_from(input)?, expected?, "{}", msg);
        }

        Ok(())
    }
}
