use anyhow::Result;
use ktxstats::v3::KtxstatsV3;

use crate::players;
use crate::players::players_from_ktxstats;
use crate::team::{teams_from_players, Team};

pub fn teams(data: &[u8]) -> Result<Vec<Team>> {
    Ok(teams_from_players(&players(data)?))
}

pub fn teams_from_ktxstats(stats: &KtxstatsV3) -> Result<Vec<Team>> {
    Ok(teams_from_players(&players_from_ktxstats(stats)?))
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use crate::ktxstats_v3;
    use crate::player::Player;

    use super::*;

    #[test]
    fn test_teams() -> Result<()> {
        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd");
            assert_eq!(
                teams(&demo_data?)?,
                vec![
                    Team {
                        name: "oeks".to_string(),
                        color: [0, 1,],
                        frags: 118,
                        ping: 26,
                        players: vec![
                            Player {
                                name: "tim.........áøå".to_string(),
                                team: "oeks".to_string(),
                                color: [0, 1,],
                                frags: 33,
                                ping: 26,
                                is_bot: false,
                                auth_username: None,
                                auth_cc: None,
                            },
                            Player {
                                name: "tco.........áøå".to_string(),
                                team: "oeks".to_string(),
                                color: [0, 1,],
                                frags: 32,
                                ping: 26,
                                is_bot: false,
                                auth_username: None,
                                auth_cc: None,
                            },
                            Player {
                                name: "bar.........áøå".to_string(),
                                team: "oeks".to_string(),
                                color: [0, 1,],
                                frags: 27,
                                ping: 25,
                                is_bot: false,
                                auth_username: None,
                                auth_cc: None,
                            },
                            Player {
                                name: "trl.........áøå".to_string(),
                                team: "oeks".to_string(),
                                color: [0, 1,],
                                frags: 26,
                                ping: 28,
                                is_bot: false,
                                auth_username: None,
                                auth_cc: None,
                            },
                        ],
                    },
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
                                auth_username: None,
                                auth_cc: None,
                            },
                            Player {
                                name: "djevulsk".to_string(),
                                team: "tSÑ".to_string(),
                                color: [11, 10,],
                                frags: 74,
                                ping: 12,
                                is_bot: false,
                                auth_username: None,
                                auth_cc: None,
                            },
                            Player {
                                name: "conan".to_string(),
                                team: "tSÑ".to_string(),
                                color: [11, 10,],
                                frags: 71,
                                ping: 12,
                                is_bot: false,
                                auth_username: None,
                                auth_cc: None,
                            },
                            Player {
                                name: "elguapo".to_string(),
                                team: "tSÑ".to_string(),
                                color: [11, 10,],
                                frags: 60,
                                ping: 13,
                                is_bot: false,
                                auth_username: None,
                                auth_cc: None,
                            },
                        ],
                    },
                ]
            );
        }

        Ok(())
    }

    #[test]
    fn test_teams_from_ktxstats() -> Result<()> {
        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;

            assert_eq!(
                teams(&demo_data)?,
                teams_from_ktxstats(&ktxstats_v3(&demo_data)?)?
            );
        }

        Ok(())
    }
}
