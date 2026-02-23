use anyhow::{anyhow as e, Result};
use bstr::ByteSlice;
pub use ktxstats::v3::KtxstatsV3;

use crate::qw::prot::HiddenMessage;
use crate::qw::{block, frame};

pub fn ktxstats_v3(data: &[u8]) -> Result<KtxstatsV3> {
    let stats_str = ktxstats_string(data)?;
    ktxstats::v3::KtxstatsV3::try_from(stats_str.as_str()).map_err(|err| e!(err))
}

pub fn ktxstats_string(data: &[u8]) -> Result<String> {
    const TOTAL_HEADER_SIZE: usize = frame::MULTI_HEADER_SIZE + block::HEADER_SIZE;

    let Some(mut offset) = data.find(br#"{"version": "#) else {
        return Err(e!("ktxstats not found"));
    };
    offset -= TOTAL_HEADER_SIZE;
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

    Ok(String::from_utf8(content)?)
}

#[cfg(test)]
mod tests {
    use std::fs::{read, read_to_string};

    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use serde_json::Value;

    use super::*;

    #[test]
    fn test_ktxstats_v3() -> Result<()> {
        let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        let stats = ktxstats_v3(&demo_data).unwrap();

        assert_eq!(stats.version, 3);
        assert_eq!(stats.hostname, "QUAKE.SE KTX:28502".to_string());

        Ok(())
    }

    fn to_pretty_json(input: &str) -> Result<String> {
        let value: Value = serde_json::from_str(input)?;
        let pretty = serde_json::to_string_pretty(&value)?;
        Ok(pretty)
    }

    #[test]
    fn test_ktxstats_string() -> Result<()> {
        {
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            let expected =
                read_to_string("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd.ktxstats.json")?;
            assert_eq!(
                to_pretty_json(&ktxstats_string(&demo_data)?)?,
                to_pretty_json(&expected)?
            );
        }
        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            let expected = read_to_string(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd.ktxstats.json",
            )?;
            assert_eq!(
                to_pretty_json(&ktxstats_string(&demo_data)?)?,
                to_pretty_json(&expected)?
            );
        }
        {
            let demo_data = read("tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd")?;
            assert_eq!(
                ktxstats_string(&demo_data).unwrap_err().to_string(),
                "ktxstats not found"
            );
        }
        {
            let demo_data = read("tests/files/20260220-0409_4on4_pex_vs_red[dm3].mvd")?;
            let expected =
                read_to_string("tests/files/20260220-0409_4on4_pex_vs_red[dm3].mvd.ktxstats.json")?;
            assert_eq!(
                to_pretty_json(&ktxstats_string(&demo_data)?)?,
                to_pretty_json(&expected)?
            );
        }

        Ok(())
    }
}
