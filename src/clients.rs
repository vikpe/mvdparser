use anyhow::Result;
use quake_text::unicode;

use crate::client::Client;
use crate::clientinfo;

pub fn clients(data: &[u8]) -> Result<Vec<Client>> {
    let clients: Vec<Client> = clientinfo::clientinfo(data)?
        .iter()
        .enumerate()
        .map(|(number, info)| {
            let mut client = Client::from(info);
            client.number = number as u8;
            client
        })
        .collect();
    Ok(clients)
}

pub fn has_bot_players(data: &[u8]) -> Result<bool> {
    Ok(player_clients(data)?.iter().any(|c| c.is_bot))
}

pub fn has_human_players(data: &[u8]) -> Result<bool> {
    Ok(player_clients(data)?.iter().any(|c| !c.is_bot))
}

pub fn player_clients(data: &[u8]) -> Result<Vec<Client>> {
    let players = clients(data)?
        .iter()
        .filter(|c| !c.is_spectator)
        .cloned()
        .collect();
    Ok(players)
}

pub fn player_names(data: &[u8]) -> Result<Vec<String>> {
    let names = player_clients(data)?
        .iter()
        .map(|c| c.name.clone())
        .collect::<Vec<String>>();
    Ok(unicode::sort(&names))
}

pub fn spectator_clients(data: &[u8]) -> Result<Vec<Client>> {
    let spectators = clients(data)?
        .iter()
        .filter(|c| c.is_spectator)
        .cloned()
        .collect();
    Ok(spectators)
}

pub fn spectator_names(data: &[u8]) -> Result<Vec<String>> {
    let names: Vec<String> = spectator_clients(data)?
        .iter()
        .map(|c| c.name.clone())
        .collect();
    Ok(unicode::sort(&names))
}

pub fn team_names(data: &[u8]) -> Result<Vec<String>> {
    let mut names: Vec<String> = player_clients(data)?
        .iter()
        .map(|c| c.team.clone())
        .collect();

    names = unicode::sort(&names);
    names.dedup();
    Ok(names)
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_clients() -> Result<()> {
        assert_eq!(
            clients(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?,
            vec![
                Client {
                    number: 0,
                    name: "eQu".to_string(),
                    team: "red".to_string(),
                    color: [4, 4],
                    is_spectator: false,
                    is_bot: false,
                    auth_username: None,
                    auth_cc: None,
                },
                Client {
                    number: 1,
                    name: "[ServeMe]".to_string(),
                    team: "lqwc".to_string(),
                    color: [12, 11],
                    is_spectator: true,
                    is_bot: false,
                    auth_username: None,
                    auth_cc: None,
                },
                Client {
                    number: 2,
                    name: "KabÏÏm".to_string(),
                    team: "".to_string(),
                    color: [2, 2],
                    is_spectator: false,
                    is_bot: false,
                    auth_username: None,
                    auth_cc: None,
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_has_bot_players() -> Result<()> {
        {
            let demo_data = read("tests/files/ffa_5[dm4]20240501-1229.mvd")?;
            assert!(has_bot_players(&demo_data)?);
        }
        {
            let demo_data = read("tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd")?;
            assert!(has_bot_players(&demo_data)?);
        }
        {
            let demo_data = read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd")?;
            assert!(!has_bot_players(&demo_data)?);
        }
        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            assert!(!has_bot_players(&demo_data)?);
        }

        Ok(())
    }

    #[test]
    fn test_has_human_players() -> Result<()> {
        {
            let demo_data = read("tests/files/ffa_5[dm4]20240501-1229.mvd")?;
            assert!(has_human_players(&demo_data)?);
        }
        {
            let demo_data = read("tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd")?;
            assert!(has_human_players(&demo_data)?);
        }
        {
            let demo_data = read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd")?;
            assert!(has_human_players(&demo_data)?);
        }
        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            assert!(has_human_players(&demo_data)?);
        }

        Ok(())
    }

    #[test]
    fn test_player_clients() -> Result<()> {
        assert_eq!(
            player_clients(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?,
            vec![
                Client {
                    number: 0,
                    name: "eQu".to_string(),
                    team: "red".to_string(),
                    color: [4, 4],
                    is_spectator: false,
                    is_bot: false,
                    auth_username: None,
                    auth_cc: None,
                },
                Client {
                    number: 2,
                    name: "KabÏÏm".to_string(),
                    team: "".to_string(),
                    color: [2, 2],
                    is_spectator: false,
                    is_bot: false,
                    auth_username: None,
                    auth_cc: None,
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_spectator_clients() -> Result<()> {
        assert_eq!(
            spectator_clients(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?,
            vec![Client {
                number: 1,
                name: "[ServeMe]".to_string(),
                team: "lqwc".to_string(),
                color: [12, 11],
                is_spectator: true,
                is_bot: false,
                auth_username: None,
                auth_cc: None,
            },]
        );

        Ok(())
    }

    #[test]
    fn test_spectator_names() -> Result<()> {
        assert_eq!(
            spectator_names(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?,
            vec!["[ServeMe]".to_string()]
        );

        Ok(())
    }

    #[test]
    fn test_player_names() -> Result<()> {
        assert_eq!(
            player_names(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?,
            vec!["eQu".to_string(), "KabÏÏm".to_string()]
        );

        assert_eq!(
            player_names(&read(
                "tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd"
            )?)?,
            vec![
                ": Sujoy".to_string(),
                ": Timber".to_string(),
                "> MrJustice".to_string(),
                "Final".to_string(),
            ]
        );

        Ok(())
    }

    #[test]
    fn test_team_names() -> Result<()> {
        assert_eq!(
            team_names(&read(
                "tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd"
            )?)?,
            vec!["=SF=".to_string(), "red".to_string()]
        );

        assert_eq!(
            team_names(&read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?)?,
            vec!["blue".to_string(), "red".to_string()]
        );

        assert_eq!(
            team_names(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?)?,
            vec!["oeks".to_string(), "tSÑ".to_string()]
        );

        assert_eq!(
            team_names(&read(
                "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd"
            )?)?,
            vec!["blue".to_string(), "red".to_string()]
        );

        Ok(())
    }
}
