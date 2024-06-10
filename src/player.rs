use std::cmp::Ordering;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Player {
    pub name: String,
    pub team: String,
    pub color: [u8; 2],
    pub frags: i32,
    pub ping: u32,
    pub is_bot: bool,
    pub auth_username: Option<String>,
    pub auth_cc: Option<String>,
}

impl From<&ktxstats::v3::Player> for Player {
    fn from(player: &ktxstats::v3::Player) -> Self {
        let auth_username = match !player.login.is_empty() {
            true => Some(player.login.clone()),
            false => None,
        };

        Self {
            name: player.name.clone(),
            team: player.team.clone(),
            color: [player.top_color as u8, player.bottom_color as u8],
            frags: player.stats.frags,
            ping: player.ping as u32,
            is_bot: player.ping == 10,
            auth_username,
            auth_cc: None,
        }
    }
}

pub fn sort() -> fn(&Player, &Player) -> Ordering {
    |b, a| a.frags.cmp(&b.frags).then(b.name.cmp(&a.name))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_player_from_ktxplayer() {
        let ktxplayer = ktxstats::v3::Player {
            name: "Alpha".to_string(),
            team: "red".to_string(),
            stats: ktxstats::v3::PlayerStats {
                frags: 54,
                ..Default::default()
            },
            top_color: 4,
            bottom_color: 3,
            ping: 25,
            ..Default::default()
        };

        assert_eq!(
            Player::from(&ktxplayer),
            Player {
                name: "Alpha".to_string(),
                team: "red".to_string(),
                color: [4, 3],
                frags: 54,
                ping: 25,
                is_bot: false,
                auth_username: None,
                auth_cc: None,
            }
        );
    }
}
