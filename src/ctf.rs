use std::collections::HashMap;

use crate::all::{player_clients, player_flag_events, PlayerFlagEvents};

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

pub fn points(data: &[u8]) -> anyhow::Result<HashMap<String, i32>> {
    let players = player_clients(data)?;
    let mut points_pp = HashMap::from_iter(players.iter().cloned().map(|c| (c.name.clone(), 0)));

    let events_pp = player_flag_events(data)?;

    let flagsdata: HashMap<String, (String, CtfPoints)> =
        HashMap::from_iter(players.iter().cloned().map(|c| {
            let default_events = PlayerFlagEvents::default();
            let events = events_pp.get(&c.name.clone()).unwrap_or(&default_events);
            let scores = CtfPoints::from(events);
            (c.name.clone(), (c.team.clone(), scores))
        }));

    for (name, (team, scores)) in flagsdata {
        if let Some(player_frags) = points_pp.get_mut(&name) {
            *player_frags += scores.player;
        }

        for teammate in players
            .iter()
            .filter(|&c| c.team == team && c.name != name)
            .cloned()
            .map(|c| c.name)
        {
            if let Some(player_frags) = points_pp.get_mut(&teammate) {
                *player_frags += scores.team;
            }
        }
    }

    Ok(points_pp)
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CtfPoints {
    pub player: i32,
    pub team: i32,
}

impl From<&PlayerFlagEvents> for CtfPoints {
    fn from(value: &PlayerFlagEvents) -> Self {
        let mut scores = CtfPoints::default();
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

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_points() -> Result<()> {
        let demo_data = read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?;
        let points_pp = points(&demo_data)?;
        assert_eq!(points_pp.len(), 10);
        assert_eq!(points_pp.get("CCTãáöåòïî"), Some(&8));
        assert_eq!(points_pp.get("CCTâéìì"), Some(&9));
        assert_eq!(points_pp.get("CCTÓèéîéîç"), Some(&4));
        assert_eq!(points_pp.get("CCTäêåöõìóë"), Some(&0));
        assert_eq!(points_pp.get("CCTÈåíìïãë"), Some(&5));
        assert_eq!(points_pp.get("ì÷ú\u{AD}velocity"), Some(&141));
        assert_eq!(points_pp.get("ì÷ú\u{AD}lethalwiz"), Some(&120));
        assert_eq!(points_pp.get("ì÷ú\u{AD}lag"), Some(&103));
        assert_eq!(points_pp.get("ì÷ú\u{AD}xunito"), Some(&105));
        assert_eq!(points_pp.get("lwz-brunelson"), Some(&105));

        Ok(())
    }

    #[test]
    fn test_ctf_points() {
        assert_eq!(
            CtfPoints::from(&PlayerFlagEvents {
                defends: 3,
                carrier_frags: 2,
                returns: 2,
                ..Default::default()
            }),
            CtfPoints {
                player: 15,
                team: 0,
            }
        );
    }
}
