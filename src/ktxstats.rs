use std::str::from_utf8;

use bstr::ByteSlice;
use ktxstats::v3::KtxstatsV3;

use crate::frame;

const JSON_NEEDLE: &[u8; 12] = br#"{"version": "#;

pub fn ktxstats_v3(data: &[u8]) -> Option<KtxstatsV3> {
    let stats_str = ktxstats_string(data)?;
    ktxstats::v3::KtxstatsV3::try_from(stats_str.as_str()).ok()
}

pub fn ktxstats_string(data: &[u8]) -> Option<String> {
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

    #[test]
    fn test_ktxstats_v3() -> Result<()> {
        let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        let stats = ktxstats_v3(&demo_data).unwrap();

        assert_eq!(stats.version, 3);
        assert_eq!(stats.hostname, "QUAKE.SE KTX:28502".to_string());

        Ok(())
    }

    fn strip(value: &str) -> String {
        value
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    }

    #[test]
    fn test_ktxstats_string() -> Result<()> {
        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            let stats_str = strip(&read_to_string(
                "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd.ktxstats.json",
            )?);
            assert_eq!(
                ktxstats_string(&demo_data).map(|s| strip(&s)),
                Some(strip(&stats_str))
            );
        }
        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            let stats_str = strip(&read_to_string(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd.ktxstats.json",
            )?);
            assert_eq!(
                ktxstats_string(&demo_data).map(|s| strip(&s)),
                Some(strip(&stats_str))
            );
        }
        {
            let demo_data = read("tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd")?;
            assert_eq!(ktxstats_string(&demo_data), None)
        }

        Ok(())
    }
}
