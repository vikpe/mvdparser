use std::collections::HashMap;

use crate::player::Player;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Team {
    pub name: String,
    pub color: [u8; 2],
    pub frags: i32,
    pub ping: u32,
    pub players: Vec<Player>,
}

impl From<&[Player]> for Team {
    fn from(players: &[Player]) -> Self {
        if players.is_empty() {
            return Self::default();
        }

        let mut team = Team {
            name: players[0].team.to_string(),
            players: vec![],
            frags: 0,
            ping: 0,
            color: [0, 0],
        };

        for player in players.iter() {
            team.players.push(player.clone());
            team.frags += player.frags;
            team.ping += player.ping;
        }

        team.ping = (team.ping as f32 / team.players.len() as f32).round() as u32;
        let player_colors: Vec<[u8; 2]> = team.players.iter().map(|p| p.color).collect();
        team.color = majority_color(&player_colors).unwrap_or(players[0].color);

        team
    }
}

pub fn teams_from_players(players: &[Player]) -> Vec<Team> {
    let mut tmap: HashMap<String, Vec<Player>> = HashMap::new();
    for player in players.iter() {
        let teamplayers = tmap.entry(player.team.clone()).or_default();
        teamplayers.push(player.clone());
    }

    let mut teams: Vec<Team> = vec![];

    for teamplayers in tmap.values() {
        teams.push(Team::from(teamplayers.as_slice()))
    }

    teams.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    teams
}

fn majority_color(colors: &[[u8; 2]]) -> Option<[u8; 2]> {
    match colors.len() {
        0 => return None,
        1 | 2 => return Some(colors[0]),
        _ => {}
    };

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
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_teams_from_players() {
        {
            let red_alpha = Player {
                name: "Alpha".to_string(),
                team: "red".to_string(),
                color: [4, 3],
                frags: 54,
                ping: 25,
                auth_username: None,
                auth_cc: None,
                is_bot: false,
            };
            let red_beta = Player {
                name: "Beta".to_string(),
                team: "red".to_string(),
                color: [4, 3],
                frags: 16,
                ping: 12,
                auth_username: None,
                auth_cc: None,
                is_bot: false,
            };
            let blue_gamma = Player {
                name: "Gamma".to_string(),
                team: "blue".to_string(),
                color: [11, 10],
                frags: 29,
                ping: 52,
                auth_username: None,
                auth_cc: None,
                is_bot: false,
            };
            let players = vec![red_alpha.clone(), red_beta.clone(), blue_gamma.clone()];

            assert_eq!(
                teams_from_players(&players),
                vec![
                    Team {
                        name: "blue".to_string(),
                        color: [11, 10],
                        frags: 29,
                        ping: 52,
                        players: vec![blue_gamma,],
                    },
                    Team {
                        name: "red".to_string(),
                        color: [4, 3],
                        frags: 70,
                        ping: 19,
                        players: vec![red_alpha, red_beta,],
                    },
                ]
            );
        }
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
