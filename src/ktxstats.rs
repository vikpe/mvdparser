use std::str::from_utf8;

use bstr::ByteSlice;
use ktxstats::v3::KtxstatsV3;

use crate::qw::HiddenMessage;
use crate::{block, frame};

pub fn ktxstats_v3(data: &[u8]) -> Option<KtxstatsV3> {
    let stats_str = ktxstats_string(data)?;
    ktxstats::v3::KtxstatsV3::try_from(stats_str.as_str()).ok()
}

pub fn ktxstats_string(data: &[u8]) -> Option<String> {
    const TOTAL_HEADER_SIZE: usize = frame::MULTI_HEADER_SIZE + block::HEADER_SIZE;
    let mut offset = data.rfind(br#"{"version": "#)? - TOTAL_HEADER_SIZE;
    let mut content = Vec::new();

    // read blocks
    while let Ok(info) = block::Info::try_from(&data[offset + frame::MULTI_HEADER_SIZE..]) {
        if info.hidden_message != HiddenMessage::Demoinfo {
            break;
        }

        offset += TOTAL_HEADER_SIZE;
        content.extend_from_slice(&data[offset..offset + info.body_size]);

        if info.number == 0 {
            break;
        }

        offset += info.body_size;
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
