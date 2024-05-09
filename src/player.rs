use ktxstats::v3;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Player {
    pub name: String,
    pub team: String,
    pub color: [u8; 2],
    pub frags: i32,
    pub ping: u32,
    pub is_bot: bool,
}

impl From<&v3::Player> for Player {
    fn from(player: &v3::Player) -> Self {
        Self {
            name: player.name.clone(),
            team: player.team.clone(),
            color: [player.top_color as u8, player.bottom_color as u8],
            frags: player.stats.frags,
            ping: player.ping as u32,
            is_bot: player.ping == 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_player_from_ktxplayer() {
        let ktxplayer = v3::Player {
            name: "Alpha".to_string(),
            team: "red".to_string(),
            stats: v3::PlayerStats {
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
            }
        );
    }
}
