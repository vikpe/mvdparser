use std::str::from_utf8;

use anyhow::{anyhow as e, Result};
use bstr::ByteSlice;
use ktxstats::v3::KtxstatsV3;

use crate::qw::prot::HiddenMessage;
use crate::qw::{block, frame};

pub fn ktxstats_v3(data: &[u8]) -> Result<KtxstatsV3> {
    let stats_str = ktxstats_string(data)?;
    ktxstats::v3::KtxstatsV3::try_from(stats_str.as_str()).map_err(|err| e!(err))
}

pub fn ktxstats_string(data: &[u8]) -> Result<String> {
    const TOTAL_HEADER_SIZE: usize = frame::MULTI_HEADER_SIZE + block::HEADER_SIZE;

    let Some(mut offset) = data.rfind(br#"{"version": "#) else {
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

    Ok(from_utf8(&content)?.to_string())
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
            let expected = strip(&read_to_string(
                "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd.ktxstats.json",
            )?);
            assert_eq!(strip(&ktxstats_string(&demo_data)?), strip(&expected));
        }
        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            let expected = strip(&read_to_string(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd.ktxstats.json",
            )?);
            assert_eq!(strip(&ktxstats_string(&demo_data)?), strip(&expected));
        }
        {
            let demo_data = read("tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd")?;
            assert_eq!(
                ktxstats_string(&demo_data).unwrap_err().to_string(),
                "ktxstats not found"
            );
        }

        Ok(())
    }
}
