use anyhow::Result;
use std::io::{Read, Seek};

use crate::client::Client;
use crate::clientinfo;

pub fn clients<R>(r: &mut R) -> Result<Vec<Client>>
where
    R: Read + Seek,
{
    let clients: Vec<Client> = clientinfo::clientinfo(r)?
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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_clients() -> Result<()> {
        assert_eq!(
            clients(&mut File::open(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?,
            vec![
                Client {
                    number: 0,
                    name: "eQu".to_string(),
                    team: "red".to_string(),
                    top_color: 4,
                    bottom_color: 4,
                    is_spectator: false,
                    is_bot: false,
                    auth_username: None,
                    auth_cc: None,
                },
                Client {
                    number: 1,
                    name: "[ServeMe]".to_string(),
                    team: "lqwc".to_string(),
                    top_color: 12,
                    bottom_color: 11,
                    is_spectator: true,
                    is_bot: false,
                    auth_username: None,
                    auth_cc: None,
                },
                Client {
                    number: 2,
                    name: "KabÏÏm".to_string(),
                    team: "".to_string(),
                    top_color: 2,
                    bottom_color: 2,
                    is_spectator: false,
                    is_bot: false,
                    auth_username: None,
                    auth_cc: None,
                },
            ]
        );

        Ok(())
    }
}
