use crate::timezone;
use bstr::ByteSlice;
use chrono::{DateTime, Utc};

const NEEDLE: &[u8; 11] = b"matchdate: ";
const DATETIME_LEN: usize = "yyyy-mm-dd hh:mm:ss".len();
const MIN_LEN: usize = "yyyy-mm-dd hh:mm:ss ab".len();
const MAX_LEN: usize = "yyyy-mm-dd hh:mm:ss abcde".len();

pub fn matchdate(data: &[u8]) -> Option<DateTime<Utc>> {
    let index_from = data.find(NEEDLE)? + NEEDLE.len();
    let index_to = index_from + data[index_from..].find_byte(b'\n')?;
    let length = index_to - index_from;

    if !(MIN_LEN..=MAX_LEN).contains(&length) {
        return None;
    }

    let Ok(matchdate_with_tz_abbr) = String::from_utf8(data[index_from..index_to].to_vec()) else {
        return None;
    };

    let matchdate_with_tz_offset = {
        let tz_abbr = &matchdate_with_tz_abbr[DATETIME_LEN + 1..];
        matchdate_with_tz_abbr.replace(tz_abbr, &timezone::utc_offset(tz_abbr)?)
    };

    match DateTime::parse_from_str(&matchdate_with_tz_offset, "%Y-%m-%d %H:%M:%S%z") {
        Ok(dt) => Some(dt.to_utc()),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chrono::DateTime;
    use pretty_assertions::assert_eq;

    use super::*;

    fn to_utc_opt(value: &str) -> Option<DateTime<Utc>> {
        Some(DateTime::parse_from_rfc3339(value).unwrap().to_utc())
    }

    #[test]
    fn test_matchdate() -> Result<()> {
        // invalid
        assert_eq!(matchdate(b""), None);
        assert_eq!(matchdate(b"foo"), None);
        assert_eq!(matchdate(b"matchdate: foo"), None);
        assert_eq!(matchdate(b"matchdate: 2024"), None);
        assert_eq!(matchdate(b"matchdate: 2024-04-02 21:02:17\n"), None);
        assert_eq!(matchdate(b"matchdate: 2024-04-02 21:02:17 FOOBAR\n"), None);

        // valid
        assert_eq!(
            matchdate(b"matchdate: 2024-04-02 21:02:17 CEST\n"),
            to_utc_opt("2024-04-02T19:02:17+00:00")
        );
        assert_eq!(
            matchdate(b"matchdate: 2024-04-02 21:02:17 GMT\n"),
            to_utc_opt("2024-04-02T21:02:17+00:00")
        );
        assert_eq!(
            matchdate(b"matchdate: 2024-04-02 21:02:17 UTC\n"),
            to_utc_opt("2024-04-02T21:02:17+00:00")
        );
        assert_eq!(
            matchdate(b"matchdate: 2024-04-02 21:02:17 -01\n"),
            to_utc_opt("2024-04-02T22:02:17+00:00")
        );
        assert_eq!(
            matchdate(b"matchdate: 2024-04-02 21:02:17 +0200\n"),
            to_utc_opt("2024-04-02T19:02:17+00:00")
        );

        // files
        {
            let data = std::fs::read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            assert_eq!(matchdate(&data), to_utc_opt("2024-04-26 16:59:29+02:00"))
        }

        {
            let data = std::fs::read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
            assert_eq!(matchdate(&data), to_utc_opt("2024-04-26 17:16:13+02:00"));
        }

        Ok(())
    }
}
