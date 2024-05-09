use quake_clientinfo::Clientinfo;

use crate::clientinfo;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Client {
    pub number: u8,
    pub name: String,
    pub team: String,
    pub color: [u8; 2],
    // todo: pub frags: i32,
    // todo: pub ping: i32,
    pub is_spectator: bool,
    pub is_bot: bool,
}

impl From<&Clientinfo> for Client {
    fn from(value: &Clientinfo) -> Self {
        Client {
            number: 0,
            name: value.name.clone().unwrap_or_default(),
            team: value.team.clone().unwrap_or_default(),
            color: [
                value.topcolor.unwrap_or_default() as u8,
                value.bottomcolor.unwrap_or_default() as u8,
            ],
            is_spectator: value.spectator.is_some_and(|v| v != 0),
            is_bot: value.bot.is_some_and(|v| v != 0),
        }
    }
}

pub fn clients(data: &[u8]) -> Option<Vec<Client>> {
    let clients: Vec<Client> = clientinfo::clientinfo(data)?
        .iter()
        .enumerate()
        .map(|(number, info)| {
            let mut client = Client::from(info);
            client.number = number as u8;
            client
        })
        .collect();
    Some(clients)
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_clientinfo() -> Result<()> {
        assert_eq!(
            clients(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?),
            Some(vec![
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
            ])
        );

        Ok(())
    }
}
