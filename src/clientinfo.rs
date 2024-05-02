use bstr::ByteSlice;

// find strings between "\" and [0] appearing after "cmd spawn"
fn clientinfo_bstrings(data: &[u8]) -> Vec<Vec<u8>> {
    let Some(o_spawn) = data.find(b"cmd spawn") else {
        return vec![];
    };

    const MIN_LEN: usize = r#"\client\ "#.len();
    const MAX_LEN: usize = 150;
    let mut result: Vec<Vec<u8>> = vec![];
    let mut offset = o_spawn;

    while let Some(from) = data[offset..].find(br#"\"#).map(|o| offset + o) {
        let Some(to) = data[from..].find([0]).map(|o| from + o) else {
            break;
        };

        if !(MIN_LEN..=MAX_LEN).contains(&(to - from)) {
            break;
        }

        result.push(data[from..to].to_vec());
        offset = to;
    }

    result
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    fn readable_bstrings(bstrings: &[Vec<u8>]) -> Vec<String> {
        bstrings
            .iter()
            .map(|b| String::from_utf8_lossy(b).to_string())
            .collect()
    }

    #[test]
    fn test_clientinfo_string() -> Result<()> {
        {
            let bstrings = clientinfo_bstrings(&read("tests/files/ffa_5[dm4]20240501-1229.mvd")?);
            assert_eq!(
                readable_bstrings(&bstrings),
                vec![
                    r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\lqwc\name\[ServeMe]"#.to_string(),
                    r#"\*client\ezQuake 1\bottomcolor\0\topcolor\0\team\sdf\name\test"#.to_string(),
                    r#"\chat\1\*client\ezQuake 1\*spectator\1\bottomcolor\1\topcolor\0\skin\oeks_nig\team\oeks\name\nig.........���"#.to_string(),
                    r#"\*bot\1\bottomcolor\6\topcolor\0\skin\base\name\/ bro"#.to_string(),
                    r#"\chat\2\*spectator\1\*client\ezQuake 1\gender\m\bottomcolor\1\topcolor\2\team\oeks\name\Z"#.to_string(),
                    r#"\*bot\1\bottomcolor\13\topcolor\3\skin\base\name\/ goldenboy"#.to_string(),
                    r#"\*bot\1\bottomcolor\11\topcolor\10\skin\base\name\/ tincan"#.to_string(),
                    r#"\*bot\1\bottomcolor\4\topcolor\3\skin\base\name\/ grue"#.to_string(),
                ]
            );
        }

        {
            let bstrings = clientinfo_bstrings(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd",
            )?);
            assert_eq!(
                readable_bstrings(&bstrings),
                vec![
                    r#"\*client\ezQuake 1\gender\m\bottomcolor\4\topcolor\4\team\red\name\eQu"#.to_string(),
                    r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\lqwc\name\[ServeMe]"#.to_string(),
                    r#"\chat\1\*client\ezQuake 1\gender\m\bottomcolor\2\topcolor\2\name\Kab��m"#.to_string(),
                ]
            );
        }

        {
            let bstrings = clientinfo_bstrings(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd",
            )?);
            assert_eq!(
                readable_bstrings(&bstrings),
                vec![
                    r#"\*client\ezQuake 1\bottomcolor\4\topcolor\4\team\x\name\HoLy"#.to_string(),
                    r#"\*client\ezQuake 1\*qwfwd\1.2\bottomcolor\4\topcolor\4\team\red\name\����"#.to_string(),
                    r#"\chat\2\*client\ezQuake 1\*spectator\1\gender\m\bottomcolor\4\topcolor\4\team\red\name\Quake"#.to_string(),
                    r#"\chat\2\*client\ezQuake 1\*spectator\1\bottomcolor\3\topcolor\1\team\mix\name\����"#.to_string(),
                    r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\lqwc\name\[ServeMe]"#.to_string(),
                ]
            );
        }

        {
            let bstrings =
                clientinfo_bstrings(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?);
            assert_eq!(
                readable_bstrings(&bstrings),
                vec![
                    r#"\*client\ezQuake 7139\bottomcolor\1\topcolor\0\skin\oeks_tco\team\oeks\name\tco.........���"#.to_string(),
                    r#"\chat\1\*client\ezQuake 1\*qwfwd\1.2\bottomcolor\1\topcolor\0\skin\oeks_bar\team\oeks\name\bar.........���"#.to_string(),
                    r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\lqwc\name\[ServeMe]"#.to_string(),
                    r#"\*client\ezQuake 1\bottomcolor\10\topcolor\11\team\tS�\name\elguapo"#.to_string(),
                    r#"\*client\ezQuake 7190\bottomcolor\1\topcolor\0\skin\oeks_trl\team\oeks\name\trl.........���"#.to_string(),
                    r#"\*client\ezQuake 1\bottomcolor\10\topcolor\11\team\tS�\name\conan"#.to_string(),
                    r#"\*client\ezQuake 1\bottomcolor\10\topcolor\11\skin\base\team\tS�\name\muttan"#.to_string(),
                    r#"\*client\ezQuake 1\*spectator\1\bottomcolor\10\topcolor\11\team\tS�\name\nas"#.to_string(),
                    r#"\chat\1\team\tS�\gender\m\topcolor\11\bottomcolor\10\*client\ezQuake 1\name\djevulsk"#.to_string(),
                    r#"\chat\2\*client\ezQuake 1\bottomcolor\1\topcolor\0\skin\oeks_tim\team\oeks\name\tim.........���"#.to_string(),
                    r#"\chat\1\*client\ezQuake 1\*spectator\1\bottomcolor\4\topcolor\4\team\red\name\lakso"#.to_string(),
                ]
            );
        }

        {
            let bstrings = clientinfo_bstrings(&read(
                "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd",
            )?);
            assert_eq!(
                readable_bstrings(&bstrings),
                vec![
                    r#"\*client\ezQuake 1\bottomcolor\4\topcolor\4\team\red\name\z0mbie90"#.to_string(),
                    r#"\*client\ezQuake 0\gender\m\bottomcolor\13\topcolor\13\team\blue\name\Kalle Dangerous"#.to_string(),
                    r#"\chat\1\team\blue\*client\ezQuake 1\gender\m\bottomcolor\13\topcolor\13\name\j0rmund"#.to_string(),
                    r#"\*client\ezQuake 7683\bottomcolor\0\topcolor\0\team\red\name\lu��"#.to_string(),
                    r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\lqwc\name\[ServeMe]"#.to_string(),
                    r#"\*client\ezQuake 1\bottomcolor\4\topcolor\4\team\blue\name\grotzky"#.to_string(),
                ]
            );
        }

        Ok(())
    }
}
