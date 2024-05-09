use std::collections::HashMap;

use anyhow::Result;

use crate::clients::clients;
use crate::{frags, pings};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Player {
    pub name: String,
    pub team: String,
    pub color: [u8; 2],
    pub frags: i32,
    pub ping: u32,
    pub is_bot: bool,
}

pub fn players(data: &[u8]) -> Result<Vec<Player>> {
    let clients = clients(data)?;
    let pings = pings(data)?;
    let frags = frags(data);
    let mut pmap: HashMap<u8, Player> = HashMap::new();

    for c in clients.iter().filter(|c| !c.is_spectator) {
        let player = Player {
            name: c.name.clone(),
            team: c.team.clone(),
            color: c.color,
            frags: *frags.get(&c.name).unwrap_or(&0),
            ping: *pings.get(&c.number).unwrap_or(&0),
            is_bot: c.is_bot,
        };
        pmap.insert(c.number, player);
    }

    let mut players: Vec<Player> = pmap.values().cloned().collect();
    players.sort_by(|b, a| a.frags.cmp(&b.frags).then(b.name.cmp(&a.name)));

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
            let demo_data = read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd");
            assert_eq!(
                players(&demo_data?)?,
                vec![
                    Player {
                        name: "KabÏÏm".to_string(),
                        team: "".to_string(),
                        color: [2, 2],
                        frags: 20,
                        ping: 29,
                        is_bot: false,
                    },
                    Player {
                        name: "eQu".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 19,
                        ping: 25,
                        is_bot: false,
                    },
                ]
            );
        }

        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd");
            assert_eq!(
                players(&demo_data?)?,
                vec![
                    Player {
                        name: "äáçï".to_string(),
                        team: "red".to_string(),
                        color: [4, 4],
                        frags: 31,
                        ping: 26,
                        is_bot: false,
                    },
                    Player {
                        name: "HoLy".to_string(),
                        team: "x".to_string(),
                        color: [4, 4],
                        frags: 25,
                        ping: 25,
                        is_bot: false,
                    },
                ]
            );
        }

        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd");
            assert_eq!(
                players(&demo_data?)?,
                vec![
                    Player {
                        name: "muttan".to_string(),
                        team: "tSÑ".to_string(),
                        color: [11, 10],
                        frags: 89,
                        ping: 12,
                        is_bot: false,
                    },
                    Player {
                        name: "djevulsk".to_string(),
                        team: "tSÑ".to_string(),
                        color: [11, 10],
                        frags: 74,
                        ping: 12,
                        is_bot: false,
                    },
                    Player {
                        name: "conan".to_string(),
                        team: "tSÑ".to_string(),
                        color: [11, 10],
                        frags: 71,
                        ping: 12,
                        is_bot: false,
                    },
                    Player {
                        name: "elguapo".to_string(),
                        team: "tSÑ".to_string(),
                        color: [11, 10],
                        frags: 60,
                        ping: 12,
                        is_bot: false,
                    },
                    Player {
                        name: "tim.........áøå".to_string(),
                        team: "oeks".to_string(),
                        color: [0, 1],
                        frags: 33,
                        ping: 26,
                        is_bot: false,
                    },
                    Player {
                        name: "tco.........áøå".to_string(),
                        team: "oeks".to_string(),
                        color: [0, 1],
                        frags: 32,
                        ping: 26,
                        is_bot: false,
                    },
                    Player {
                        name: "bar.........áøå".to_string(),
                        team: "oeks".to_string(),
                        color: [0, 1],
                        frags: 27,
                        ping: 26,
                        is_bot: false,
                    },
                    Player {
                        name: "trl.........áøå".to_string(),
                        team: "oeks".to_string(),
                        color: [0, 1],
                        frags: 26,
                        ping: 28,
                        is_bot: false,
                    },
                ]
            );
        }

        {
            let demo_data = read("tests/files/ffa_5[dm4]20240501-1229.mvd");
            assert_eq!(
                players(&demo_data?)?,
                vec![
                    Player {
                        name: "/ tincan".to_string(),
                        team: "".to_string(),
                        color: [10, 11],
                        frags: 8,
                        ping: 10,
                        is_bot: true,
                    },
                    Player {
                        name: "/ bro".to_string(),
                        team: "".to_string(),
                        color: [0, 6],
                        frags: 6,
                        ping: 10,
                        is_bot: true,
                    },
                    Player {
                        name: "/ grue".to_string(),
                        team: "".to_string(),
                        color: [3, 4],
                        frags: 6,
                        ping: 10,
                        is_bot: true,
                    },
                    Player {
                        name: "/ goldenboy".to_string(),
                        team: "".to_string(),
                        color: [3, 13],
                        frags: 5,
                        ping: 10,
                        is_bot: true,
                    },
                    Player {
                        name: "test".to_string(),
                        team: "sdf".to_string(),
                        color: [0, 0],
                        frags: 4,
                        ping: 12,
                        is_bot: false,
                    },
                ]
            );
        }

        Ok(())
    }
}
