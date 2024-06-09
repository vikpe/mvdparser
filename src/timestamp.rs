use anyhow::{anyhow as e, Result};
use chrono::{DateTime, LocalResult, TimeZone, Utc};

use crate::matchdate::matchdate;
use crate::serverinfo;

pub fn timestamp(data: &[u8]) -> Result<DateTime<Utc>> {
    timestamp_from_epoch(data).or_else(|_| matchdate(data))
}

pub fn timestamp_from_epoch(data: &[u8]) -> Result<DateTime<Utc>> {
    let Some(epoch) = serverinfo(data)?.epoch else {
        return Err(e!("Epoch not found in serverinfo"));
    };

    match Utc.timestamp_opt(epoch as i64, 0) {
        LocalResult::Single(ts) => Ok(ts),
        LocalResult::Ambiguous(earliest, _) => Ok(earliest),
        _ => Err(e!("Unable to parse timestamp from epoch")),
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use chrono::DateTime;
    use pretty_assertions::assert_eq;

    use super::*;

    fn to_timestamp(str: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(str).unwrap().to_utc()
    }

    #[test]
    fn test_timestamp() -> Result<()> {
        assert_eq!(
            timestamp(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?)?,
            to_timestamp("2024-04-26T14:59:29Z")
        );

        assert_eq!(
            timestamp(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?,
            to_timestamp("2024-04-22T10:38:20Z")
        );

        Ok(())
    }

    #[test]
    fn test_timestamp_from_epoch() -> Result<()> {
        assert_eq!(
            timestamp_from_epoch(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?)
            .unwrap_err()
            .to_string(),
            "Epoch not found in serverinfo".to_string()
        );

        assert_eq!(
            timestamp_from_epoch(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?,
            to_timestamp("2024-04-22T10:38:20Z")
        );

        Ok(())
    }
}
