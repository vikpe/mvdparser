use std::str::from_utf8;

use bstr::ByteSlice;

const JSON_NEEDLE: &[u8; 12] = br#"{"version": "#;
const FRAME_DELIMITER: &[u8; 4] = b"\x00\x03\x00\x00";

mod frame_info {
    pub const LENGTH: usize = 18;
    pub const LENGTH_INDEX: usize = 10;
}

pub fn ktxstats(data: &[u8]) -> Option<String> {
    let mut offset = data.rfind(JSON_NEEDLE)? - frame_info::LENGTH;
    let mut content = Vec::new();

    while &data[offset..offset + FRAME_DELIMITER.len()] == FRAME_DELIMITER {
        let index_from = offset + frame_info::LENGTH;
        let index_to = index_from + get_frame_length(data, offset);
        content.extend_from_slice(&data[index_from..index_to]);
        offset = index_to;
    }

    match from_utf8(&content) {
        Ok(str) => Some(str.to_string()),
        Err(_) => None,
    }
}

fn get_frame_length(data: &[u8], offset: usize) -> usize {
    let index = offset + frame_info::LENGTH_INDEX;
    u16::from_le_bytes([data[index], data[index + 1]]) as usize - 2
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
            let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            let stats_data = strip(&read_to_string(
                "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd.ktxstats.json",
            )?);

            assert_eq!(
                ktxstats(&demo_data).map(|s| strip(&s)),
                Some(strip(&stats_data))
            );
        }

        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            let stats_data = strip(&read_to_string(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd.ktxstats.json",
            )?);

            assert_eq!(
                ktxstats(&demo_data).map(|s| strip(&s)),
                Some(strip(&stats_data))
            );
        }
        Ok(())
    }
}
