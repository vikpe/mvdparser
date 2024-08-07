use std::collections::HashMap;

use anyhow::Result;
use ktxstats::v3::KtxstatsV3;

use crate::clients::clients;
use crate::frags::frags_per_player_name;
use crate::ktxstats::ktxstats_v3;
use crate::ping::ping_per_player_number;
use crate::player;
use crate::player::Player;

pub fn players(data: &[u8]) -> Result<Vec<Player>> {
    if let Ok(stats) = ktxstats_v3(data) {
        players_from_ktxstats(&stats)
    } else {
        players_from_parsing(data)
    }
}

pub fn players_from_ktxstats(stats: &KtxstatsV3) -> Result<Vec<Player>> {
    let mut players: Vec<Player> = stats.players.iter().map(Player::from).collect();
    players.sort_by(player::sort());
    Ok(players)
}

pub fn players_from_parsing(data: &[u8]) -> Result<Vec<Player>> {
    let clients = clients(data)?;
    let pings = ping_per_player_number(data)?;
    let frags = frags_per_player_name(data);
    let mut pmap: HashMap<u8, Player> = HashMap::new();

    for c in clients.iter().filter(|c| !c.is_spectator) {
        let player = Player {
            name: c.name.clone(),
            team: c.team.clone(),
            color: c.color,
            frags: *frags.get(&c.name).unwrap_or(&0),
            ping: *pings.get(&c.number).unwrap_or(&0),
            auth_username: c.auth_username.clone(),
            auth_cc: c.auth_cc.clone(),
            is_bot: c.is_bot,
        };
        pmap.insert(c.number, player);
    }

    let mut players: Vec<Player> = pmap.values().cloned().collect();
    players.sort_by(player::sort());
    Ok(players)
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
            let demo_data = read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd")?;
            let from_parsing = players_from_parsing(&demo_data)?;
            let from_ktxstats = players_from_ktxstats(&ktxstats_v3(&demo_data)?)?;

            assert_eq!(from_parsing.len(), from_ktxstats.len());
            for n in 0..from_parsing.len() {
                assert_eq!(from_parsing[n].frags, from_ktxstats[n].frags);
                assert_eq!(from_parsing[n].name, from_ktxstats[n].name);
                assert_eq!(from_parsing[n].team, from_ktxstats[n].team);
                assert_eq!(from_parsing[n].color, from_ktxstats[n].color);
                assert!(from_parsing[n].ping.abs_diff(from_ktxstats[n].ping) < 5);
                assert_eq!(from_parsing[n].is_bot, from_ktxstats[n].is_bot);
            }
        }
        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            let from_parsing = players_from_parsing(&demo_data)?;
            let from_ktxstats = players_from_ktxstats(&ktxstats_v3(&demo_data)?)?;

            assert_eq!(from_parsing.len(), from_ktxstats.len());
            for n in 0..from_parsing.len() {
                assert_eq!(from_parsing[n].frags, from_ktxstats[n].frags);
                assert_eq!(from_parsing[n].name, from_ktxstats[n].name);
                assert_eq!(from_parsing[n].team, from_ktxstats[n].team);
                assert_eq!(from_parsing[n].color, from_ktxstats[n].color);
                assert!(from_parsing[n].ping.abs_diff(from_ktxstats[n].ping) < 5);
                assert_eq!(from_parsing[n].is_bot, from_ktxstats[n].is_bot);
            }
        }
        {
            let demo_data = read("tests/files/ffa_5[dm4]20240501-1229.mvd")?;
            let from_parsing = players_from_parsing(&demo_data)?;
            let from_ktxstats = players_from_ktxstats(&ktxstats_v3(&demo_data)?)?;

            assert_eq!(from_parsing.len(), from_ktxstats.len());
            for n in 0..from_parsing.len() {
                assert_eq!(from_parsing[n].frags, from_ktxstats[n].frags);
                assert_eq!(from_parsing[n].name, from_ktxstats[n].name);
                assert_eq!(from_parsing[n].team, from_ktxstats[n].team);
                assert_eq!(from_parsing[n].color, from_ktxstats[n].color);
                assert!(from_parsing[n].ping.abs_diff(from_ktxstats[n].ping) < 5);
                assert_eq!(from_parsing[n].is_bot, from_ktxstats[n].is_bot);
            }
        }

        Ok(())
    }

    #[test]
    fn test_players_from_parsing() -> Result<()> {
        {
            let demo_data = read("tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd");
            assert_eq!(
                players_from_parsing(&demo_data?)?,
                vec![
                    Player {
                        name: ": Timber".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 13,
                        ping: 10,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: true,
                    },
                    Player {
                        name: ": Sujoy".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 7,
                        ping: 10,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: true,
                    },
                    Player {
                        name: "Final".to_string(),
                        team: "=SF=".to_string(),
                        color: [0, 4],
                        frags: 1,
                        ping: 51,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "> MrJustice".to_string(),
                        team: "=SF=".to_string(),
                        color: [0, 4],
                        frags: -5,
                        ping: 10,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: true,
                    },
                ]
            );
        }
        {
            let demo_data = read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd");
            assert_eq!(
                players_from_parsing(&demo_data?)?,
                vec![
                    Player {
                        name: "KabÏÏm".to_string(),
                        team: "".to_string(),
                        color: [2, 2],
                        frags: 20,
                        ping: 29,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "eQu".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 19,
                        ping: 25,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                ]
            );
        }
        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd");
            assert_eq!(
                players_from_parsing(&demo_data?)?,
                vec![
                    Player {
                        name: "äáçï".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 31,
                        ping: 26,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "HoLy".to_string(),
                        team: "x".to_string(),
                        color: [4, 4],
                        frags: 25,
                        ping: 25,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                ]
            );
        }
        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd");
            assert_eq!(
                players_from_parsing(&demo_data?)?,
                vec![
                    Player {
                        name: "muttan".to_string(),
                        team: "tSÑ".to_string(),
                        color: [11, 10],
                        frags: 89,
                        ping: 12,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "djevulsk".to_string(),
                        team: "tSÑ".to_string(),
                        color: [11, 10],
                        frags: 74,
                        ping: 12,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "conan".to_string(),
                        team: "tSÑ".to_string(),
                        color: [11, 10],
                        frags: 71,
                        ping: 12,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "elguapo".to_string(),
                        team: "tSÑ".to_string(),
                        color: [11, 10],
                        frags: 60,
                        ping: 12,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "tim.........áøå".to_string(),
                        team: "oeks".to_string(),
                        color: [0, 1],
                        frags: 33,
                        ping: 26,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "tco.........áøå".to_string(),
                        team: "oeks".to_string(),
                        color: [0, 1],
                        frags: 32,
                        ping: 26,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "bar.........áøå".to_string(),
                        team: "oeks".to_string(),
                        color: [0, 1],
                        frags: 27,
                        ping: 26,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "trl.........áøå".to_string(),
                        team: "oeks".to_string(),
                        color: [0, 1],
                        frags: 26,
                        ping: 28,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                ]
            );
        }
        {
            let demo_data = read("tests/files/ffa_5[dm4]20240501-1229.mvd");
            assert_eq!(
                players_from_parsing(&demo_data?)?,
                vec![
                    Player {
                        name: "/ tincan".to_string(),
                        team: "".to_string(),
                        color: [10, 11],
                        frags: 8,
                        ping: 10,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: true,
                    },
                    Player {
                        name: "/ bro".to_string(),
                        team: "".to_string(),
                        color: [0, 6],
                        frags: 6,
                        ping: 10,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: true,
                    },
                    Player {
                        name: "/ grue".to_string(),
                        team: "".to_string(),
                        color: [3, 4],
                        frags: 6,
                        ping: 10,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: true,
                    },
                    Player {
                        name: "/ goldenboy".to_string(),
                        team: "".to_string(),
                        color: [3, 13],
                        frags: 5,
                        ping: 10,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: true,
                    },
                    Player {
                        name: "test".to_string(),
                        team: "sdf".to_string(),
                        color: [0, 0],
                        frags: 4,
                        ping: 12,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                ]
            );
        }

        Ok(())
    }

    #[test]
    fn test_players_from_ktxstats() -> Result<()> {
        {
            let demo_data = read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd")?;
            assert_eq!(
                players_from_ktxstats(&ktxstats_v3(&demo_data)?)?,
                vec![
                    Player {
                        name: "KabÏÏm".to_string(),
                        team: "".to_string(),
                        color: [2, 2],
                        frags: 20,
                        ping: 25,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "eQu".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 19,
                        ping: 26,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                ]
            );
        }
        {
            let demo_data = read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?;
            assert_eq!(
                players_from_ktxstats(&ktxstats_v3(&demo_data)?)?,
                vec![
                    Player {
                        name: "ì÷ú\u{ad}velocity".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 164,
                        ping: 33,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "ì÷ú\u{ad}lethalwiz".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 140,
                        ping: 12,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "ì÷ú\u{ad}xunito".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 128,
                        ping: 42,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "lwz-brunelson".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 120,
                        ping: 77,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "ì÷ú\u{ad}lag".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 118,
                        ping: 40,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "\u{10}CCT\u{11}\u{9c}ãáöåòïî".to_string(),
                        team: "blue".to_string(),
                        color: [13, 13],
                        frags: 29,
                        ping: 16,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "\u{10}CCT\u{11}\u{9c}âéìì".to_string(),
                        team: "blue".to_string(),
                        color: [13, 13],
                        frags: 23,
                        ping: 12,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "\u{10}CCT\u{11}\u{9c}Óèéîéîç".to_string(),
                        team: "blue".to_string(),
                        color: [13, 13],
                        frags: 19,
                        ping: 12,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "\u{10}CCT\u{11}\u{9c}äêåöõìóë".to_string(),
                        team: "blue".to_string(),
                        color: [13, 13],
                        frags: 15,
                        ping: 12,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                    Player {
                        name: "\u{10}CCT\u{11}\u{9c}Èåíìïãë".to_string(),
                        team: "blue".to_string(),
                        color: [13, 13],
                        frags: 10,
                        ping: 46,
                        auth_username: None,
                        auth_cc: None,
                        is_bot: false,
                    },
                ]
            );
        }
        Ok(())
    }
}
