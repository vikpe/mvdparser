use anyhow::{anyhow as e, Result};
use bstr::ByteSlice;
use chrono::{DateTime, Utc};

use crate::timezone;

pub const MATCHDATE_NEEDLE: [u8; 0x0D] = [
    0x08, 0x02, 0x6D, 0x61, 0x74, 0x63, 0x68, 0x64, 0x61, 0x74, 0x65, 0x3A, 0x20,
]; // "[print] matchdate: "
const DATETIME_LEN: usize = "yyyy-mm-dd hh:mm:ss".len();
const MIN_LEN: usize = "yyyy-mm-dd hh:mm:ss ab".len();
const MAX_LEN: usize = "yyyy-mm-dd hh:mm:ss abcde".len();

pub fn matchdate(data: &[u8]) -> Result<DateTime<Utc>> {
    let raw_str = matchdate_string(data)?;
    let fixed_str = replace_tz_abbr_with_offset(&raw_str)?;
    Ok(DateTime::parse_from_str(&fixed_str, "%Y-%m-%d %H:%M:%S%z")?.to_utc())
}

fn replace_tz_abbr_with_offset(timestamp: &str) -> Result<String> {
    let tz_abbr = &timestamp[DATETIME_LEN + 1..];
    let Some(tz_offset) = timezone::utc_offset(tz_abbr) else {
        return Err(e!("Invalid timezone abbreviation"));
    };
    Ok(format!("{}{}", &timestamp[..DATETIME_LEN], tz_offset))
}

pub fn matchdate_string(data: &[u8]) -> Result<String> {
    let Some(mut index_from) = data.find(MATCHDATE_NEEDLE) else {
        return Err(e!("Matchdate not found"));
    };
    index_from += MATCHDATE_NEEDLE.len();

    let Some(mut index_to) = data[index_from..].find_byte(b'\n') else {
        return Err(e!("Invalid matchdate"));
    };
    index_to += index_from;

    let length = index_to - index_from;

    if !(MIN_LEN..=MAX_LEN).contains(&length) {
        return Err(e!("Invalid matchdate"));
    }

    Ok(String::from_utf8(data[index_from..index_to].to_vec())?)
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use chrono::DateTime;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_replace_tz_abbr_with_offset() -> Result<()> {
        assert_eq!(
            replace_tz_abbr_with_offset("2024-04-02 21:02:17 CEST")?,
            "2024-04-02 21:02:17+02:00".to_string()
        );
        assert_eq!(
            replace_tz_abbr_with_offset("2024-04-02 21:02:17 GMT")?,
            "2024-04-02 21:02:17+00:00".to_string()
        );
        assert_eq!(
            replace_tz_abbr_with_offset("2024-04-02 21:02:17 UTC")?,
            "2024-04-02 21:02:17+00:00".to_string()
        );
        assert_eq!(
            replace_tz_abbr_with_offset("2024-04-02 21:02:17 -01")?,
            "2024-04-02 21:02:17-01:00".to_string()
        );
        assert_eq!(
            replace_tz_abbr_with_offset("2024-04-02 21:02:17 +0200")?,
            "2024-04-02 21:02:17+02:00".to_string()
        );

        Ok(())
    }

    #[test]
    fn test_matchdate() -> Result<()> {
        assert_eq!(
            matchdate(b"matchdate: 2024-04-02 21:02:17 CEST\n")?,
            DateTime::parse_from_rfc3339("2024-04-02T19:02:17+00:00")
                .unwrap()
                .to_utc()
        );

        Ok(())
    }

    #[test]
    fn test_matchdate_string() -> Result<()> {
        // invalid
        assert_eq!(
            matchdate_string(b"").unwrap_err().to_string(),
            "Matchdate not found".to_string()
        );
        assert_eq!(
            matchdate_string(b"foo").unwrap_err().to_string(),
            "Matchdate not found".to_string()
        );
        assert_eq!(
            matchdate_string(b"matchdate: foo").unwrap_err().to_string(),
            "Invalid matchdate".to_string()
        );
        assert_eq!(
            matchdate_string(b"matchdate: 2024")
                .unwrap_err()
                .to_string(),
            "Invalid matchdate".to_string()
        );
        assert_eq!(
            matchdate_string(b"matchdate: 2024-04-02 21:02:17\n")
                .unwrap_err()
                .to_string(),
            "Invalid matchdate".to_string()
        );
        assert_eq!(
            matchdate_string(b"matchdate: 2024-04-02 21:02:17 FOOBAR\n")
                .unwrap_err()
                .to_string(),
            "Invalid matchdate".to_string()
        );

        // valid
        assert_eq!(
            matchdate_string(b"matchdate: 2024-04-02 21:02:17 CEST\n")?,
            "2024-04-02 21:02:17 CEST".to_string()
        );

        // files
        assert_eq!(
            matchdate_string(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?)?,
            "2024-04-26 16:59:29 CEST".to_string()
        );

        assert_eq!(
            matchdate_string(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?)?,
            "2024-04-26 17:16:13 CEST".to_string()
        );

        assert_eq!(
            matchdate_string(&read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?)?,
            "2024-05-20 19:25:42 UTC".to_string()
        );

        Ok(())
    }
}
