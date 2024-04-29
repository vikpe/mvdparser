use bstr::ByteSlice;
use chrono::{DateTime, Utc};

use crate::timezone;

const NEEDLE: &[u8; 11] = b"matchdate: ";
const DATETIME_LEN: usize = "yyyy-mm-dd hh:mm:ss".len();
const MIN_LEN: usize = "yyyy-mm-dd hh:mm:ss ab".len();
const MAX_LEN: usize = "yyyy-mm-dd hh:mm:ss abcde".len();

pub fn matchdate(data: &[u8]) -> Option<DateTime<Utc>> {
    let matchdate_str = replace_tz_abbr_with_offset(&matchdate_string(data)?)?;

    match DateTime::parse_from_str(&matchdate_str, "%Y-%m-%d %H:%M:%S%z") {
        Ok(dt) => Some(dt.to_utc()),
        Err(_) => None,
    }
}

pub fn replace_tz_abbr_with_offset(timestamp: &str) -> Option<String> {
    let tz_abbr = &timestamp[DATETIME_LEN + 1..];
    let tz_offset = timezone::utc_offset(tz_abbr)?;
    Some(format!("{}{}", &timestamp[..DATETIME_LEN], tz_offset))
}

pub fn matchdate_string(data: &[u8]) -> Option<String> {
    let index_from = data.find(NEEDLE)? + NEEDLE.len();
    let index_to = index_from + data[index_from..].find_byte(b'\n')?;
    let length = index_to - index_from;

    if !(MIN_LEN..=MAX_LEN).contains(&length) {
        return None;
    }

    String::from_utf8(data[index_from..index_to].to_vec()).ok()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chrono::DateTime;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_replace_tz_abbr_with_offset() {
        assert_eq!(
            replace_tz_abbr_with_offset("2024-04-02 21:02:17 CEST"),
            Some("2024-04-02 21:02:17+02:00".to_string())
        );
        assert_eq!(
            replace_tz_abbr_with_offset("2024-04-02 21:02:17 GMT"),
            Some("2024-04-02 21:02:17+00:00".to_string())
        );
        assert_eq!(
            replace_tz_abbr_with_offset("2024-04-02 21:02:17 UTC"),
            Some("2024-04-02 21:02:17+00:00".to_string())
        );
        assert_eq!(
            replace_tz_abbr_with_offset("2024-04-02 21:02:17 -01"),
            Some("2024-04-02 21:02:17-01:00".to_string())
        );
        assert_eq!(
            replace_tz_abbr_with_offset("2024-04-02 21:02:17 +0200"),
            Some("2024-04-02 21:02:17+02:00".to_string())
        );
    }

    #[test]
    fn test_matchdate() -> Result<()> {
        assert_eq!(
            matchdate(b"matchdate: 2024-04-02 21:02:17 CEST\n"),
            Some(
                DateTime::parse_from_rfc3339("2024-04-02T19:02:17+00:00")
                    .unwrap()
                    .to_utc()
            )
        );

        Ok(())
    }

    #[test]
    fn test_matchdate_string() -> Result<()> {
        // invalid
        assert_eq!(matchdate_string(b""), None);
        assert_eq!(matchdate_string(b"foo"), None);
        assert_eq!(matchdate_string(b"matchdate: foo"), None);
        assert_eq!(matchdate_string(b"matchdate: 2024"), None);
        assert_eq!(matchdate_string(b"matchdate: 2024-04-02 21:02:17\n"), None);
        assert_eq!(
            matchdate_string(b"matchdate: 2024-04-02 21:02:17 FOOBAR\n"),
            None
        );

        // valid
        assert_eq!(
            matchdate_string(b"matchdate: 2024-04-02 21:02:17 CEST\n"),
            Some("2024-04-02 21:02:17 CEST".to_string())
        );

        // files
        assert_eq!(
            matchdate_string(&std::fs::read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?),
            Some("2024-04-26 16:59:29 CEST".to_string())
        );

        assert_eq!(
            matchdate_string(&std::fs::read(
                "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd"
            )?),
            Some("2024-04-26 17:16:13 CEST".to_string())
        );

        Ok(())
    }
}
