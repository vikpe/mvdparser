use anyhow::{anyhow as e, Result};

use crate::flagprint::{
    X_CAPTURED_FLAG, X_DEFENDS_FLAG, X_DEFENDS_FLAG_CARRIER, X_DEFENDS_FLAG_CARRIER_VS_AGGRESSIVE,
    X_GOT_FLAG, X_RETURNED_FLAG, X_RETURNED_FLAG_ASSIST,
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FlagEvent {
    CapturedFlag { player: String },
    GotFlag { player: String },
    DefendsFlag { player: String },
    DefendsFlagCarrier { player: String },
    DefendFlagCarrierVsAggressive { player: String },
    ReturnedFlag { player: String },
    ReturnedFlagAssist { player: String },
}

impl TryFrom<&[u8]> for FlagEvent {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let text = quake_text::bytestr::to_utf8(value);
        let text = text.trim_end();

        if let Some(pos) = X_CAPTURED_FLAG.iter().find_map(|p| text.find(*p)) {
            let player = quake_text::bytestr::to_unicode(&value[..pos]);
            return Ok(FlagEvent::CapturedFlag { player });
        } else if let Some(pos) = X_DEFENDS_FLAG.iter().find_map(|s| text.find(*s)) {
            let player = quake_text::bytestr::to_unicode(&value[..pos]);
            return Ok(FlagEvent::DefendsFlag { player });
        } else if let Some(pos) = X_DEFENDS_FLAG_CARRIER_VS_AGGRESSIVE
            .iter()
            .find_map(|s| text.find(*s))
        {
            let player = quake_text::bytestr::to_unicode(&value[..pos]);
            return Ok(FlagEvent::DefendFlagCarrierVsAggressive { player });
        } else if let Some(pos) = X_DEFENDS_FLAG_CARRIER.iter().find_map(|s| text.find(*s)) {
            let player = quake_text::bytestr::to_unicode(&value[..pos]);
            return Ok(FlagEvent::DefendsFlagCarrier { player });
        } else if let Some(pos) = X_GOT_FLAG.iter().find_map(|s| text.find(*s)) {
            let player = quake_text::bytestr::to_unicode(&value[..pos]);
            return Ok(FlagEvent::GotFlag { player });
        } else if let Some(pos) = X_RETURNED_FLAG.iter().find_map(|s| text.find(*s)) {
            let player = quake_text::bytestr::to_unicode(&value[..pos]);
            return Ok(FlagEvent::ReturnedFlag { player });
        } else if let Some(pos) = X_RETURNED_FLAG_ASSIST.iter().find_map(|s| text.find(*s)) {
            let player = quake_text::bytestr::to_unicode(&value[..pos]);
            return Ok(FlagEvent::ReturnedFlagAssist { player });
        }

        Err(e!(r#"Unable to parse message 2: "{}""#, text))
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
                "FOO got the RED flag!\n",
                Ok(FlagEvent::GotFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO got the ÒÅÄ flag!\n",
                Ok(FlagEvent::GotFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO got the BLUE flag!\n",
                Ok(FlagEvent::GotFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO got the ÂÌÕÅ flag!\n",
                Ok(FlagEvent::GotFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO captured the RED flag!\n",
                Ok(FlagEvent::CapturedFlag {
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
                "FOO captured the BLUE flag!\n",
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
                "FOO gets an assist for returning his flag!\n",
                Ok(FlagEvent::ReturnedFlagAssist {
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
                "FOO returned the RED flag!\n",
                Ok(FlagEvent::ReturnedFlag {
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
                "FOO returned the BLUE flag!\n",
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
                "FOO defends the RED flag\n",
                Ok(FlagEvent::DefendsFlag {
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
                "FOO defends the BLUE flag\n",
                Ok(FlagEvent::DefendsFlag {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends RED's flag carrier\n",
                Ok(FlagEvent::DefendsFlagCarrier {
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
                "FOO defends BLUE's flag carrier\n",
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
                "FOO defends RED's flag carrier against an aggressive enemy\n",
                Ok(FlagEvent::DefendFlagCarrierVsAggressive {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends ÒÅÄ's flag carrier against an aggressive enemy\n",
                Ok(FlagEvent::DefendFlagCarrierVsAggressive {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends BLUE's flag carrier against an aggressive enemy\n",
                Ok(FlagEvent::DefendFlagCarrierVsAggressive {
                    player: "FOO".to_string(),
                }),
            ),
            (
                "FOO defends ÂÌÕÅ's flag carrier against an aggressive enemy\n",
                Ok(FlagEvent::DefendFlagCarrierVsAggressive {
                    player: "FOO".to_string(),
                }),
            ),
        ]);

        for (input_str, expected) in test_cases {
            let msg = format!(r#""{}" should equal {:?}"#, &input_str, &expected);
            let input_bytes = quake_text::unicode::to_bytestr(input_str);
            assert_eq!(FlagEvent::try_from(&input_bytes[..])?, expected?, "{}", msg);
        }

        Ok(())
    }
}
