use anyhow::Result;

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

pub fn player_clients(data: &[u8]) -> Result<Vec<Client>> {
    let players = clients(data)?
        .iter()
        .filter(|c| !c.is_spectator)
        .cloned()
        .collect();
    Ok(players)
}

pub fn spectator_clients(data: &[u8]) -> Result<Vec<Client>> {
    let spectators = clients(data)?
        .iter()
        .filter(|c| c.is_spectator)
        .cloned()
        .collect();
    Ok(spectators)
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
                },
                Client {
                    number: 1,
                    name: "[ServeMe]".to_string(),
                    team: "lqwc".to_string(),
                    color: [12, 11],
                    is_spectator: true,
                    is_bot: false,
                },
                Client {
                    number: 2,
                    name: "KabÏÏm".to_string(),
                    team: "".to_string(),
                    color: [2, 2],
                    is_spectator: false,
                    is_bot: false,
                },
            ]
        );

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
                },
                Client {
                    number: 2,
                    name: "KabÏÏm".to_string(),
                    team: "".to_string(),
                    color: [2, 2],
                    is_spectator: false,
                    is_bot: false,
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
            },]
        );

        Ok(())
    }
}
