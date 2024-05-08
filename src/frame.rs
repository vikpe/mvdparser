use std::ops::Range;

use anyhow::{anyhow as e, Result};

use crate::num;
use crate::qw::{Command, Target};

pub const HEADER_SIZE: usize = num::size::SHORT + num::size::LONG;
pub const MULTI_HEADER_SIZE: usize = HEADER_SIZE + num::size::LONG;

mod index {
    pub const DURATION: usize = 0;
    pub const TARGET: usize = 1;
    pub const COMMAND: usize = 1;
    pub const SIZE: usize = 2;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub duration: u32,
    pub target: Target,
    pub command: Command,
    pub index: usize,
    pub size: usize,
    pub header_size: usize,
    pub header_range: Range<usize>,
    pub body_size: usize,
    pub body_range: Range<usize>,
}

impl Info {
    pub fn from_data(data: &[u8]) -> Result<Self> {
        Self::from_data_and_index(data, 0)
    }

    pub fn from_data_and_index(data: &[u8], index: usize) -> Result<Self> {
        let data = &data[index..];

        if data.len() < MULTI_HEADER_SIZE {
            return Err(e!("frame::Info: unsufficient length"));
        }

        let target = Target::from(&data[index::TARGET]);
        let command = Command::from(&data[index::COMMAND]);

        let skipped_bytes = match target {
            Target::Multiple => num::size::LONG, // skip multi target bytes [0,0,0,0]
            _ => 0,
        };

        let header_size = HEADER_SIZE + skipped_bytes;
        let body_size = match command {
            Command::Read => num::long(&data[skipped_bytes + index::SIZE..]) as usize,
            Command::Set => 2 * num::size::LONG, // reads 2 longs (8 bytes)
            _ => 0,                              // should not happen
        };
        let size = header_size + body_size;

        Ok(Self {
            index,
            duration: data[index::DURATION] as u32,
            target,
            command,
            size,
            header_size,
            header_range: index..index + header_size,
            body_size,
            body_range: index + header_size..index + size,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SliceInfo {
    pub size: usize,
    pub range_abs: Range<usize>,
    // pub range_rel: Range<usize>,
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_info_from_data() -> Result<()> {
        let data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;

        let expected = Info {
            index: 456,
            duration: 0,
            target: Target::All,
            command: Command::Read,
            size: 743,
            header_size: 6,
            header_range: 456..462,
            body_size: 737,
            body_range: 462..1199,
        };

        assert_eq!(Info::from_data_and_index(&data, 456)?, expected);
        assert_eq!(
            Info::from_data(&data)?,
            Info::from_data_and_index(&data, 0)?,
        );

        Ok(())
    }
}
