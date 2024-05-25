use anyhow::{anyhow as e, Result};

use crate::flagprint::{
    X_CAPTURED_FLAG, X_DEFENDS_FLAG, X_DEFENDS_FLAG_CARRIER, X_DEFENDS_FLAG_CARRIER_VS_AGGRESSIVE,
    X_GOT_FLAG, X_RETURNED_FLAG, X_RETURNED_FLAG_ASSIST,
};

#[derive(Debug, PartialEq, Eq)]
pub enum FlagEvent {
    CaptureFlag { player: String },
    GotFlag { player: String },
    DefendFlag { player: String },
    DefendFlagCarrier { player: String },
    DefendFlagCarrierVsAggressive { player: String },
    ReturnFlag { player: String },
    ReturnFlagAssist { player: String },
}

impl TryFrom<&str> for FlagEvent {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(p) = X_CAPTURED_FLAG.iter().find(|&p| value.ends_with(p)) {
            return Ok(FlagEvent::CaptureFlag {
                player: value.trim_end_matches(p).to_string(),
            });
        } else if let Some(p) = X_RETURNED_FLAG_ASSIST.iter().find(|&p| value.ends_with(p)) {
            return Ok(FlagEvent::ReturnFlagAssist {
                player: value.trim_end_matches(p).to_string(),
            });
        } else if let Some(p) = X_RETURNED_FLAG.iter().find(|&p| value.ends_with(p)) {
            return Ok(FlagEvent::ReturnFlag {
                player: value.trim_end_matches(p).to_string(),
            });
        } else if let Some(p) = X_DEFENDS_FLAG.iter().find(|&p| value.ends_with(p)) {
            return Ok(FlagEvent::DefendFlag {
                player: value.trim_end_matches(p).to_string(),
            });
        } else if let Some(p) = X_DEFENDS_FLAG_CARRIER.iter().find(|&p| value.ends_with(p)) {
            return Ok(FlagEvent::DefendFlagCarrier {
                player: value.trim_end_matches(p).to_string(),
            });
        } else if let Some(p) = X_DEFENDS_FLAG_CARRIER_VS_AGGRESSIVE
            .iter()
            .find(|&p| value.ends_with(p))
        {
            return Ok(FlagEvent::DefendFlagCarrierVsAggressive {
                player: value.trim_end_matches(p).to_string(),
            });
        } else if let Some(p) = X_GOT_FLAG.iter().find(|&p| value.ends_with(p)) {
            return Ok(FlagEvent::GotFlag {
                player: value.trim_end_matches(p).to_string(),
            });
        }

        Err(e!(r#"Unable to parse message: "{}""#, value))
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
                "FOO got the RED flag!",
                Ok(FlagEvent::GotFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO got the ÒÅÄ flag!",
                Ok(FlagEvent::GotFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO got the BLUE flag!",
                Ok(FlagEvent::GotFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO got the ÂÌÕÅ flag!",
                Ok(FlagEvent::GotFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO captured the RED flag!",
                Ok(FlagEvent::CaptureFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO ãáðôõòåä the ÒÅÄ flag!",
                Ok(FlagEvent::CaptureFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO captured the BLUE flag!",
                Ok(FlagEvent::CaptureFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO ãáðôõòåä the ÂÌÕÅ flag!",
                Ok(FlagEvent::CaptureFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO gets an assist for returning his flag!",
                Ok(FlagEvent::ReturnFlagAssist {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO gets an assist for fragging the flag carrier!",
                Ok(FlagEvent::ReturnFlagAssist {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO returned the RED flag!",
                Ok(FlagEvent::ReturnFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO returned the ÒÅÄ flag!",
                Ok(FlagEvent::ReturnFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO returned the BLUE flag!",
                Ok(FlagEvent::ReturnFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO returned the ÂÌÕÅ flag!",
                Ok(FlagEvent::ReturnFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends the RED flag",
                Ok(FlagEvent::DefendFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends the ÒÅÄ flag",
                Ok(FlagEvent::DefendFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends the BLUE flag",
                Ok(FlagEvent::DefendFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends RED's flag carrier",
                Ok(FlagEvent::DefendFlagCarrier {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends ÒÅÄ's flag carrier",
                Ok(FlagEvent::DefendFlagCarrier {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends BLUE's flag carrier",
                Ok(FlagEvent::DefendFlagCarrier {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends ÂÌÕÅ's flag carrier",
                Ok(FlagEvent::DefendFlagCarrier {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends RED's flag carrier against an aggressive enemy",
                Ok(FlagEvent::DefendFlagCarrierVsAggressive {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends ÒÅÄ's flag carrier against an aggressive enemy",
                Ok(FlagEvent::DefendFlagCarrierVsAggressive {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends BLUE's flag carrier against an aggressive enemy",
                Ok(FlagEvent::DefendFlagCarrierVsAggressive {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends ÂÌÕÅ's flag carrier against an aggressive enemy",
                Ok(FlagEvent::DefendFlagCarrierVsAggressive {
                    player: "FOO".to_string(),
                }),
            ),
        ]);

        for (input, expected) in test_cases {
            let msg = format!(r#""{}" should equal {:?}"#, &input, &expected);
            assert_eq!(FlagEvent::try_from(input)?, expected?, "{}", msg);
        }

        Ok(())
    }
}
