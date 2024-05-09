use std::collections::HashMap;

use anyhow::Result;

use crate::players;
use crate::players::Player;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Team {
    pub name: String,
    pub color: [u8; 2],
    pub frags: i32,
    pub ping: u32,
    pub players: Vec<Player>,
}

pub fn teams(data: &[u8]) -> Result<Vec<Team>> {
    let players = players(data)?;

    let mut tmap: HashMap<String, Team> = HashMap::new();
    for player in players.iter() {
        let team = tmap.entry(player.team.clone()).or_insert(Team {
            name: player.team.clone(),
            players: vec![],
            frags: 0,
            ping: 0,
            color: [0, 0],
        });

        team.players.push(player.clone());
        team.frags += player.frags;
        team.ping += player.ping;
    }

    let mut teams: Vec<Team> = tmap.values().cloned().collect();
    teams.sort_by(|b, a| a.frags.cmp(&b.frags));

    for t in teams.iter_mut() {
        t.ping = (t.ping as f32 / t.players.len() as f32).round() as u32;
        let player_colors: Vec<[u8; 2]> = t.players.iter().map(|p| p.color).collect();
        t.color = majority_color(&player_colors).unwrap_or(players[0].color);
    }

    Ok(teams)
}

fn majority_color(colors: &[[u8; 2]]) -> Option<[u8; 2]> {
    let mut color_count: HashMap<[u8; 2], u8> = HashMap::new();
    for color in colors.iter() {
        let count = color_count.entry(*color).or_insert(0);
        *count += 1;
    }

    let mut max_color = None;
    let mut max_count = 0;
    for (color, count) in color_count.iter() {
        if *count > max_count {
            max_color = Some(*color);
            max_count = *count;
        }
    }

    max_color
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_players() -> Result<()> {
        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd");
            assert_eq!(
                teams(&demo_data?)?,
                vec![
                    Team {
                        name: "tSÑ".to_string(),
                        color: [11, 10,],
                        frags: 294,
                        ping: 12,
                        players: vec![
                            Player {
                                name: "muttan".to_string(),
                                team: "tSÑ".to_string(),
                                color: [11, 10,],
                                frags: 89,
                                ping: 12,
                                is_bot: false,
                            },
                            Player {
                                name: "djevulsk".to_string(),
                                team: "tSÑ".to_string(),
                                color: [11, 10,],
                                frags: 74,
                                ping: 12,
                                is_bot: false,
                            },
                            Player {
                                name: "conan".to_string(),
                                team: "tSÑ".to_string(),
                                color: [11, 10,],
                                frags: 71,
                                ping: 12,
                                is_bot: false,
                            },
                            Player {
                                name: "elguapo".to_string(),
                                team: "tSÑ".to_string(),
                                color: [11, 10,],
                                frags: 60,
                                ping: 12,
                                is_bot: false,
                            },
                        ],
                    },
                    Team {
                        name: "oeks".to_string(),
                        color: [0, 1,],
                        frags: 118,
                        ping: 27,
                        players: vec![
                            Player {
                                name: "tim.........áøå".to_string(),
                                team: "oeks".to_string(),
                                color: [0, 1,],
                                frags: 33,
                                ping: 26,
                                is_bot: false,
                            },
                            Player {
                                name: "tco.........áøå".to_string(),
                                team: "oeks".to_string(),
                                color: [0, 1,],
                                frags: 32,
                                ping: 26,
                                is_bot: false,
                            },
                            Player {
                                name: "bar.........áøå".to_string(),
                                team: "oeks".to_string(),
                                color: [0, 1,],
                                frags: 27,
                                ping: 26,
                                is_bot: false,
                            },
                            Player {
                                name: "trl.........áøå".to_string(),
                                team: "oeks".to_string(),
                                color: [0, 1,],
                                frags: 26,
                                ping: 28,
                                is_bot: false,
                            },
                        ],
                    },
                ]
            );
        }

        Ok(())
    }

    #[test]
    fn test_majority_color() {
        {
            let colors = vec![[4, 3], [11, 10]];
            assert_eq!(majority_color(&colors), Some([4, 3]));
        }
        {
            let colors = vec![[4, 3], [11, 10], [0, 0], [11, 10]];
            assert_eq!(majority_color(&colors), Some([11, 10]));
        }
    }
}
