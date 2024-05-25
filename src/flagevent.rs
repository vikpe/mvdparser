use anyhow::{anyhow as e, Result};

use crate::flagmessages::{

};

#[derive(Debug, PartialEq, Eq)]
pub enum FlagEvent {
    Capture { player: String },
    Defend { player: String },
    DefendCarrier { player: String },
    DefendCarrierVsAggressive { player: String },
    ReturnFlag { player: String },
    ReturnFlagAssist { player: String },
}

impl TryFrom<&str> for FlagEvent {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {


        if let Some(p) = X_CAPTURE_FLAG.iter().find(|&p| value.ends_with(p)) {
            return Ok(FragEvent::FlagAlert {
                player: value.trim_end_matches(p).to_string(),
                event: FlagEvent::Capture,
            });
        } else if let Some(p) = X_RETURN_FLAG_ASSIST.iter().find(|&p| value.ends_with(p)) {
            return Ok(FragEvent::FlagAlert {
                player: value.trim_end_matches(p).to_string(),
                event: FlagEvent::ReturnFlagAssist,
            });
        } else if let Some(p) = X_RETURN_FLAG.iter().find(|&p| value.ends_with(p)) {
            return Ok(FragEvent::FlagAlert {
                player: value.trim_end_matches(p).to_string(),
                event: FlagEvent::ReturnFlag,
            });
        } else if let Some(p) = X_DEFEND_FLAG.iter().find(|&p| value.ends_with(p)) {
            return Ok(FragEvent::FlagAlert {
                player: value.trim_end_matches(p).to_string(),
                event: FlagEvent::Defend,
            });
        } else if let Some(p) = X_DEFEND_CARRIER.iter().find(|&p| value.ends_with(p)) {
            return Ok(FragEvent::FlagAlert {
                player: value.trim_end_matches(p).to_string(),
                event: FlagEvent::DefendCarrier,
            });
        } else if let Some(p) = X_DEFEND_CARRIER_VS_AGGRESSIVE
            .iter()
            .find(|&p| value.ends_with(p))
        {
            return Ok(FragEvent::FlagAlert {
                player: value.trim_end_matches(p).to_string(),
                event: FlagEvent::DefendCarrierVsAggressive,
            });
        }

        Err(e!(r#"Unable to parse message: "{}""#, value))
    }
}

fn pattern_match(value: &str, pattern: &str) -> Option<(String, String)> {
    let (x, y) = match pattern.contains(WILDCARD) {
        true => {
            let (prefix, suffix) = pattern.split_once(WILDCARD)?;
            let (x, rest) = value.split_once(prefix)?;
            let (y, _) = rest.split_once(suffix)?;
            (x, y)
        }
        false => value.split_once(pattern)?,
    };

    Some((x.to_string(), y.to_string()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use crate::fragevent::FragEvent::{
        Death, Frag, Suicide, SuicideByWeapon, Teamkill, TeamkillByUnknown,
    };

    use super::*;

    fn foo_death() -> Result<FragEvent> {
        Ok(Death {
            player: "FOO".to_string(),
        })
    }

    fn foo_suicide_wpn() -> Result<FragEvent> {
        Ok(SuicideByWeapon {
            player: "FOO".to_string(),
        })
    }

    fn foo_tk() -> Result<FragEvent> {
        Ok(Teamkill {
            killer: "FOO".to_string(),
        })
    }

    fn foo_tk_by_unknown() -> Result<FragEvent> {
        Ok(TeamkillByUnknown {
            victim: "FOO".to_string(),
        })
    }

    fn foo_frag_bar() -> Result<FragEvent> {
        Ok(Frag {
            killer: "FOO".to_string(),
            victim: "BAR".to_string(),
        })
    }

    fn bar_frag_foo() -> Result<FragEvent> {
        Ok(Frag {
            killer: "BAR".to_string(),
            victim: "FOO".to_string(),
        })
    }

    #[test]
    fn test_frag_event() -> Result<()> {
        let test_cases: HashMap<&str, Result<FragEvent>> = HashMap::from([
            // x death
            ("FOO sleeps with the fishes", foo_death()),
            ("FOO sucks it down", foo_death()),
            ("FOO gulped a load of slime", foo_death()),
            ("FOO can't exist on slime alone", foo_death()),
            ("FOO burst into flames", foo_death()),
            ("FOO turned into hot slag", foo_death()),
            ("FOO visits the Volcano God", foo_death()),
            ("FOO cratered", foo_death()),
            ("FOO fell to his death", foo_death()),
            ("FOO fell to her death", foo_death()),
            ("FOO blew up", foo_death()),
            ("FOO was spiked", foo_death()),
            ("FOO was zapped", foo_death()),
            ("FOO ate a lavaball", foo_death()),
            ("FOO died", foo_death()),
            ("FOO tried to leave", foo_death()),
            ("FOO was squished", foo_death()),
            //
            // x suicide by weapon
            ("FOO tries to put the pin back in", foo_suicide_wpn()),
            ("FOO becomes bored with life", foo_suicide_wpn()),
            ("FOO discovers blast radius", foo_suicide_wpn()),
            ("FOO electrocutes himself", foo_suicide_wpn()),
            ("FOO electrocutes herself", foo_suicide_wpn()),
            ("FOO railcutes himself", foo_suicide_wpn()),
            ("FOO railcutes herself", foo_suicide_wpn()),
            ("FOO discharges into the slime", foo_suicide_wpn()),
            ("FOO discharges into the lava", foo_suicide_wpn()),
            ("FOO discharges into the water", foo_suicide_wpn()),
            ("FOO heats up the water", foo_suicide_wpn()),
            //
            // x suicide
            (
                "FOO suicides",
                Ok(Suicide {
                    player: "FOO".to_string(),
                }),
            ),
            //
            // x teamkill unknown
            ("FOO squished a teammate", foo_tk()),
            ("FOO mows down a teammate", foo_tk()),
            ("FOO checks his glasses", foo_tk()),
            ("FOO checks her glasses", foo_tk()),
            ("FOO gets a frag for the other team", foo_tk()),
            ("FOO loses another friend", foo_tk()),
            //
            // unknown teamkill x
            ("FOO was telefragged by his teammate", foo_tk_by_unknown()),
            ("FOO was telefragged by her teammate", foo_tk_by_unknown()),
            ("FOO was crushed by his teammate", foo_tk_by_unknown()),
            ("FOO was crushed by her teammate", foo_tk_by_unknown()),
            ("FOO was jumped by his teammate", foo_tk_by_unknown()),
            ("FOO was jumped by her teammate", foo_tk_by_unknown()),
            //
            // X frag Y
            (r#"FOO stomps BAR"#, foo_frag_bar()),
            (r#"FOO squishes BAR"#, foo_frag_bar()),
            (r#"FOO rips BAR a new one"#, foo_frag_bar()),
            //
            // Y frag X
            (r#"FOO was ax-murdered by BAR"#, bar_frag_foo()),
            (r#"FOO softens BAR's fall"#, bar_frag_foo()),
            (r#"FOO tried to catch BAR"#, bar_frag_foo()),
            (r#"FOO was crushed by BAR"#, bar_frag_foo()),
            (r#"FOO was jumped by BAR"#, bar_frag_foo()),
            (r#"FOO chewed on BAR's boomstick"#, bar_frag_foo()),
            (r#"FOO was body pierced by BAR"#, bar_frag_foo()),
            (r#"FOO was nailed by BAR"#, bar_frag_foo()),
            (r#"FOO was railed by BAR"#, bar_frag_foo()),
            (r#"FOO was telefragged by BAR"#, bar_frag_foo()),
            (r#"FOO accepts BAR's discharge"#, bar_frag_foo()),
            (r#"FOO drains BAR's batteries"#, bar_frag_foo()),
            (r#"FOO was lead poisoned by BAR"#, bar_frag_foo()),
            (r#"FOO accepts BAR's shaft"#, bar_frag_foo()),
            (r#"FOO ate 2 loads of BAR's buckshot"#, bar_frag_foo()),
            (r#"FOO was perforated by BAR"#, bar_frag_foo()),
            (r#"FOO was punctured by BAR"#, bar_frag_foo()),
            (r#"FOO was ventilated by BAR"#, bar_frag_foo()),
            (r#"FOO ate 8 loads of BAR's buckshot"#, bar_frag_foo()),
            (r#"FOO gets a natural disaster from BAR"#, bar_frag_foo()),
            (r#"FOO rides BAR's rocket"#, bar_frag_foo()),
            (r#"FOO was gibbed by BAR's rocket"#, bar_frag_foo()),
            (r#"FOO was straw-cuttered by BAR"#, bar_frag_foo()),
            (r#"FOO eats BAR's pineapple"#, bar_frag_foo()),
            (r#"FOO was gibbed by BAR's grenade"#, bar_frag_foo()),
            (r#"FOO was brutalized by BAR's quad rocket"#, bar_frag_foo()),
            (r#"FOO was smeared by BAR's quad rocket"#, bar_frag_foo()),
            (r#"FOO was hooked by BAR"#, bar_frag_foo()),
            //
            // flag alerts
            (
                "FOO captured the RED flag!",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::Capture,
                }),
            ),
            (
                "FOO ãáðôõòåä the ÒÅÄ flag!",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::Capture,
                }),
            ),
            (
                "FOO captured the BLUE flag!",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::Capture,
                }),
            ),
            (
                "FOO ãáðôõòåä the ÂÌÕÅ flag!",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::Capture,
                }),
            ),
            (
                "FOO gets an assist for returning his flag!",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::ReturnFlagAssist,
                }),
            ),
            (
                "FOO gets an assist for fragging the flag carrier!",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::ReturnFlagAssist,
                }),
            ),
            (
                "FOO returned the RED flag!",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::ReturnFlag,
                }),
            ),
            (
                "FOO returned the ÒÅÄ flag!",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::ReturnFlag,
                }),
            ),
            (
                "FOO returned the BLUE flag!",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::ReturnFlag,
                }),
            ),
            (
                "FOO returned the ÂÌÕÅ flag!",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::ReturnFlag,
                }),
            ),
            (
                "FOO defends the RED flag",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::Defend,
                }),
            ),
            (
                "FOO defends the ÒÅÄ flag",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::Defend,
                }),
            ),
            (
                "FOO defends the BLUE flag",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::Defend,
                }),
            ),
            (
                "FOO defends RED's flag carrier",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::DefendCarrier,
                }),
            ),
            (
                "FOO defends ÒÅÄ's flag carrier",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::DefendCarrier,
                }),
            ),
            (
                "FOO defends BLUE's flag carrier",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::DefendCarrier,
                }),
            ),
            (
                "FOO defends ÂÌÕÅ's flag carrier",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::DefendCarrier,
                }),
            ),
            (
                "FOO defends RED's flag carrier against an aggressive enemy",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::DefendCarrierVsAggressive,
                }),
            ),
            (
                "FOO defends ÒÅÄ's flag carrier against an aggressive enemy",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::DefendCarrierVsAggressive,
                }),
            ),
            (
                "FOO defends BLUE's flag carrier against an aggressive enemy",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::DefendCarrierVsAggressive,
                }),
            ),
            (
                "FOO defends ÂÌÕÅ's flag carrier against an aggressive enemy",
                Ok(FragEvent::FlagAlert {
                    player: "FOO".to_string(),
                    event: FlagEvent::DefendCarrierVsAggressive,
                }),
            ),
        ]);

        for (input, expected) in test_cases {
            let msg = format!(r#""{}" should equal {:?}"#, &input, &expected);
            assert_eq!(FragEvent::try_from(input)?, expected?, "{}", msg);
        }

        Ok(())
    }
}
