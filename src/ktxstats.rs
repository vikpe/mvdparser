use std::str::from_utf8;

use bstr::ByteSlice;

use crate::frame;

const JSON_NEEDLE: &[u8; 12] = br#"{"version": "#;

pub fn ktxstats(data: &[u8]) -> Option<String> {
    let mut offset = data.rfind(JSON_NEEDLE)? - frame::info::LENGTH;
    let mut content = Vec::new();

    while &data[offset..offset + frame::DELIMITER.len()] == frame::DELIMITER {
        let index_from = offset + frame::info::LENGTH;
        let index_to = index_from + frame::length(data, offset);
        content.extend_from_slice(&data[index_from..index_to]);
        offset = index_to;
    }

    match from_utf8(&content) {
        Ok(str) => Some(str.to_string()),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{read, read_to_string};

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    fn strip(value: &str) -> String {
        value
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    }

    #[test]
    fn test_ktxstats() -> Result<()> {
        {
            let demo = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            let stats = strip(&read_to_string(
                "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd.ktxstats.json",
            )?);
            assert_eq!(ktxstats(&demo).map(|s| strip(&s)), Some(strip(&stats)));
        }
        {
            let demo = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            let stats = strip(&read_to_string(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd.ktxstats.json",
            )?);
            assert_eq!(ktxstats(&demo).map(|s| strip(&s)), Some(strip(&stats)));
        }
        {
            let demo = read("tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd")?;
            assert_eq!(ktxstats(&demo), None)
        }

        Ok(())
    }
}
