use anyhow::{anyhow as e, Result};

use crate::fragprint::{
    UNKNOWN_TEAMKILL_X, WILDCARD, X_DEATH, X_FRAG_Y, X_SUICIDE, X_SUICIDE_BY_WEAPON,
    X_TEAMKILL_UNKNOWN, Y_FRAG_X,
};

#[derive(Debug, PartialEq, Eq)]
pub enum FragEvent {
    Death { player: String },
    Suicide { player: String },
    SuicideByWeapon { player: String },
    Frag { killer: String, victim: String },
    Teamkill { killer: String },
    TeamkillByUnknown { victim: String },
}

impl TryFrom<&str> for FragEvent {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(p) = X_DEATH.iter().find(|&p| value.ends_with(p)) {
            return Ok(FragEvent::Death {
                player: value.trim_end_matches(p).to_string(),
            });
        } else if let Some(p) = X_SUICIDE_BY_WEAPON.iter().find(|&p| value.ends_with(p)) {
            return Ok(FragEvent::SuicideByWeapon {
                player: value.trim_end_matches(p).to_string(),
            });
        } else if value.ends_with(X_SUICIDE) {
            return Ok(FragEvent::Suicide {
                player: value.trim_end_matches(X_SUICIDE).to_string(),
            });
        } else if let Some(p) = X_TEAMKILL_UNKNOWN.iter().find(|&p| value.ends_with(p)) {
            return Ok(FragEvent::Teamkill {
                killer: value.trim_end_matches(p).to_string(),
            });
        } else if let Some(p) = UNKNOWN_TEAMKILL_X.iter().find(|&p| value.ends_with(p)) {
            return Ok(FragEvent::TeamkillByUnknown {
                victim: value.trim_end_matches(p).to_string(),
            });
        }

        for p in [X_FRAG_Y.to_vec(), Y_FRAG_X.to_vec()].concat() {
            if let Some((x, y)) = pattern_match(value, p) {
                let (killer, victim) = match X_FRAG_Y.contains(&p) {
                    true => (x, y),
                    false => (y, x),
                };
                return Ok(FragEvent::Frag { killer, victim });
            }
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
        ]);

        for (input, expected) in test_cases {
            let msg = format!(r#""{}" should equal {:?}"#, &input, &expected);
            assert_eq!(FragEvent::try_from(input)?, expected?, "{}", msg);
        }

        Ok(())
    }
}
