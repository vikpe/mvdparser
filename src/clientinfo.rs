use anyhow::{anyhow as e, Result};
use bstr::ByteSlice;
use quake_clientinfo::Clientinfo;

pub fn clientinfo(data: &[u8]) -> Result<Vec<Clientinfo>> {
    let info: Vec<Clientinfo> = clientinfo_strings(data)?
        .iter()
        .map(|s| Clientinfo::from(s.as_str()))
        .collect();
    Ok(info)
}

pub fn clientinfo_strings(data: &[u8]) -> Result<Vec<String>> {
    const CMD_SPAWN: [u8; 0x0A] = [0x09, 0x63, 0x6D, 0x64, 0x20, 0x73, 0x70, 0x61, 0x77, 0x6E];

    let Some(mut offset) = data.find(CMD_SPAWN) else {
        return Err(e!("Unable to find clientinfo strings"));
    };
    const MAX_PLAYERS: usize = 24;
    const MAX_LOOKAHEAD: usize = 256;
    let max_offset: usize = offset + MAX_PLAYERS * MAX_LOOKAHEAD;
    const MIN_LEN: usize = r#"\name\ "#.len();
    const MAX_LEN: usize = 256;

    let mut result: Vec<String> = vec![];

    while let Some(name_offset) = data[offset..offset + MAX_LOOKAHEAD]
        .find(br#"\name\"#)
        .map(|o| offset + o)
    {
        let Some(from) = data[..name_offset].rfind_byte(0).map(|o| o + 1) else {
            break;
        };

        let Some(to) = data[from..].find_byte(0).map(|o| from + o) else {
            break;
        };

        if !(MIN_LEN..=MAX_LEN).contains(&(to - from)) {
            break;
        }

        result.push(quake_text::bytestr::to_unicode(&data[from..to]));
        offset = to;

        if offset >= max_offset {
            break;
        }
    }

    Ok(result)
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
            clientinfo(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?,
            vec![
                Clientinfo {
                    name: Some("eQu".to_string()),
                    team: Some("red".to_string()),
                    topcolor: Some(4),
                    bottomcolor: Some(4),
                    spectator: None,
                    client: Some("ezQuake 1".to_string()),
                    bot: None,
                    chat: None,
                },
                Clientinfo {
                    name: Some("[ServeMe]".to_string()),
                    team: Some("lqwc".to_string()),
                    topcolor: Some(12),
                    bottomcolor: Some(11),
                    spectator: Some(1),
                    client: Some("libqwclient 0.1".to_string()),
                    bot: None,
                    chat: None,
                },
                Clientinfo {
                    name: Some("KabÏÏm".to_string()),
                    team: None,
                    topcolor: Some(2),
                    bottomcolor: Some(2),
                    spectator: None,
                    client: Some("ezQuake 1".to_string()),
                    bot: None,
                    chat: Some(1),
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_clientinfo_strings() -> Result<()> {
        assert_eq!(
            clientinfo_strings(&read(
                "tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd"
            )?)?,
            vec![
                r#"\chat\1\*client\ezQuake 7034\bottomcolor\4\topcolor\0\team\=SF=\name\Final"#,
                r#"\team\red\*bot\1\bottomcolor\4\topcolor\4\skin\base\name\: Timber"#,
                r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\lqwc\name\[ServeMe]"#,
                r#"\team\=SF=\*bot\1\bottomcolor\4\topcolor\0\skin\base\name\> MrJustice"#,
                r#"\team\red\*bot\1\bottomcolor\4\topcolor\4\skin\base\name\: Sujoy"#,
            ]
        );

        assert_eq!(
            clientinfo_strings(&read("tests/files/ffa_5[dm4]20240501-1229.mvd")?)?,
            vec![
                r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\lqwc\name\[ServeMe]"#.to_string(),
                r#"\*client\ezQuake 1\bottomcolor\0\topcolor\0\team\sdf\name\test"#.to_string(),
                r#"\chat\1\*client\ezQuake 1\*spectator\1\bottomcolor\1\topcolor\0\skin\oeks_nig\team\oeks\name\nig.........áøå"#.to_string(),
                r#"\*bot\1\bottomcolor\6\topcolor\0\skin\base\name\/ bro"#.to_string(),
                r#"\chat\2\*spectator\1\*client\ezQuake 1\gender\m\bottomcolor\1\topcolor\2\team\oeks\name\Z"#.to_string(),
                r#"\*bot\1\bottomcolor\13\topcolor\3\skin\base\name\/ goldenboy"#.to_string(),
                r#"\*bot\1\bottomcolor\11\topcolor\10\skin\base\name\/ tincan"#.to_string(),
                r#"\*bot\1\bottomcolor\4\topcolor\3\skin\base\name\/ grue"#.to_string(),
            ]
        );

        assert_eq!(
            clientinfo_strings(&read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd")?)?,
            vec![
                r#"\*client\ezQuake 1\gender\m\bottomcolor\4\topcolor\4\team\red\name\eQu"#.to_string(),
                r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\lqwc\name\[ServeMe]"#.to_string(),
                r#"\chat\1\*client\ezQuake 1\gender\m\bottomcolor\2\topcolor\2\name\KabÏÏm"#.to_string(),
            ]
        );

        assert_eq!(
            clientinfo_strings(&read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?)?,
            vec![
                r#"\*client\ezQuake 1\bottomcolor\4\topcolor\4\team\x\name\HoLy"#.to_string(),
                r#"\*client\ezQuake 1\*qwfwd\1.2\bottomcolor\4\topcolor\4\team\red\name\äáçï"#.to_string(),
                r#"\chat\2\*client\ezQuake 1\*spectator\1\gender\m\bottomcolor\4\topcolor\4\team\red\name\Quake"#.to_string(),
                r#"\chat\2\*client\ezQuake 1\*spectator\1\bottomcolor\3\topcolor\1\team\mix\name\âáóó"#.to_string(),
                r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\lqwc\name\[ServeMe]"#.to_string(),
            ]
        );

        assert_eq!(
            clientinfo_strings(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?)?,
            vec![
                r#"\*client\ezQuake 7139\bottomcolor\1\topcolor\0\skin\oeks_tco\team\oeks\name\tco.........áøå"#.to_string(),
                r#"\chat\1\*client\ezQuake 1\*qwfwd\1.2\bottomcolor\1\topcolor\0\skin\oeks_bar\team\oeks\name\bar.........áøå"#.to_string(),
                r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\lqwc\name\[ServeMe]"#.to_string(),
                r#"\*client\ezQuake 1\bottomcolor\10\topcolor\11\team\tSÑ\name\elguapo"#.to_string(),
                r#"\*client\ezQuake 7190\bottomcolor\1\topcolor\0\skin\oeks_trl\team\oeks\name\trl.........áøå"#.to_string(),
                r#"\*client\ezQuake 1\bottomcolor\10\topcolor\11\team\tSÑ\name\conan"#.to_string(),
                r#"\*client\ezQuake 1\bottomcolor\10\topcolor\11\skin\base\team\tSÑ\name\muttan"#.to_string(),
                r#"\*client\ezQuake 1\*spectator\1\bottomcolor\10\topcolor\11\team\tSÑ\name\nas"#.to_string(),
                r#"\chat\1\team\tSÑ\gender\m\topcolor\11\bottomcolor\10\*client\ezQuake 1\name\djevulsk"#.to_string(),
                r#"\chat\2\*client\ezQuake 1\bottomcolor\1\topcolor\0\skin\oeks_tim\team\oeks\name\tim.........áøå"#.to_string(),
                r#"\chat\1\*client\ezQuake 1\*spectator\1\bottomcolor\4\topcolor\4\team\red\name\lakso"#.to_string(),
            ]
        );

        assert_eq!(
            clientinfo_strings(&read("tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd")?)?,
            vec![
                r#"\*client\ezQuake 1\bottomcolor\4\topcolor\4\team\red\name\z0mbie90"#.to_string(),
                r#"\*client\ezQuake 0\gender\m\bottomcolor\13\topcolor\13\team\blue\name\Kalle Dangerous"#.to_string(),
                r#"\chat\1\team\blue\*client\ezQuake 1\gender\m\bottomcolor\13\topcolor\13\name\j0rmund"#.to_string(),
                r#"\*client\ezQuake 7683\bottomcolor\0\topcolor\0\team\red\name\luòñ"#.to_string(),
                r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\lqwc\name\[ServeMe]"#.to_string(),
                r#"\*client\ezQuake 1\bottomcolor\4\topcolor\4\team\blue\name\grotzky"#.to_string(),
            ]
        );

        Ok(())
    }
}
