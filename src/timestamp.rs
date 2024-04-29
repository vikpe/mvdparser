use chrono::{DateTime, LocalResult, TimeZone, Utc};

use crate::matchdate::matchdate;
use crate::serverinfo;

pub fn timestamp(data: &[u8]) -> Option<DateTime<Utc>> {
    timestamp_from_epoch(data).or_else(|| matchdate(data))
}

fn timestamp_from_epoch(data: &[u8]) -> Option<DateTime<Utc>> {
    let epoch = serverinfo(data)?.epoch?;
    match Utc.timestamp_opt(epoch as i64, 0) {
        LocalResult::Single(ts) => Some(ts),
        LocalResult::Ambiguous(earliest, _) => Some(earliest),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use chrono::DateTime;
    use pretty_assertions::assert_eq;

    use super::*;

    fn to_timestamp_opt(str: &str) -> Option<DateTime<Utc>> {
        Some(DateTime::parse_from_rfc3339(str).unwrap().to_utc())
    }

    #[test]
    fn test_timestamp() -> Result<()> {
        assert_eq!(
            timestamp(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?),
            to_timestamp_opt("2024-04-26T14:59:29Z")
        );

        assert_eq!(
            timestamp(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?),
            to_timestamp_opt("2024-04-22T10:38:20Z")
        );

        Ok(())
    }

    #[test]
    fn test_timestamp_from_epoch() -> Result<()> {
        assert_eq!(
            timestamp_from_epoch(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?),
            None
        );

        assert_eq!(
            timestamp_from_epoch(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?),
            to_timestamp_opt("2024-04-22T10:38:20Z")
        );

        Ok(())
    }
}
