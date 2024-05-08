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
    pub header_size: usize,
    pub body_size: usize,
    pub total_size: usize,
}

impl TryFrom<&[u8]> for Info {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < MULTI_HEADER_SIZE {
            return Err(e!("frame::Info: unsufficient length"));
        }

        let target = Target::from(&value[index::TARGET]);
        let command = Command::from(&value[index::COMMAND]);

        // skip multi target bytes [0,0,0,0]
        let offset = match target {
            Target::Multiple => num::size::LONG,
            _ => 0,
        };

        let body_size = match command {
            Command::Read => num::long(&value[offset + index::SIZE..]) as usize,
            Command::Set => 2 * num::size::LONG, // reads 2 longs (8 bytes)
            _ => 0,                              // should not happen
        };

        let header_size = HEADER_SIZE + offset;

        Ok(Info {
            duration: value[index::DURATION] as u32,
            target,
            command,
            header_size,
            body_size,
            total_size: header_size + body_size,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DetailedInfo {
    pub duration: u32,
    pub target: Target,
    pub command: Command,
    pub header: SliceInfo,
    pub body: SliceInfo,
    pub size: usize,
    pub range_abs: Range<usize>,
    pub range_rel: Range<usize>,
}

impl DetailedInfo {
    pub fn from_data_and_index(source_data: &[u8], index: usize) -> Result<Self> {
        let data = &source_data[index..];

        if data.len() < MULTI_HEADER_SIZE {
            return Err(e!("frame::Info: unsufficient length"));
        }

        let target = Target::from(&data[index::TARGET]);
        let command = Command::from(&data[index::COMMAND]);

        // skip multi target bytes [0,0,0,0]
        let skipped_bytes = match target {
            Target::Multiple => num::size::LONG,
            _ => 0,
        };

        // header
        let header_size = HEADER_SIZE + skipped_bytes;
        let header = SliceInfo {
            size: header_size,
            range_abs: index..index + header_size,
            range_rel: 0..header_size,
        };

        // body
        let body_size = match command {
            Command::Read => num::long(&data[skipped_bytes + index::SIZE..]) as usize,
            Command::Set => 2 * num::size::LONG, // reads 2 longs (8 bytes)
            _ => 0,                              // should not happen
        };

        let body = SliceInfo {
            size: body_size,
            range_abs: index + header_size..index + header_size + body_size,
            range_rel: header_size..header_size + body_size,
        };

        // frame
        let frame_size = header.size + body.size;
        Ok(Self {
            duration: data[index::DURATION] as u32,
            target,
            command,
            header,
            body,
            size: frame_size,
            range_abs: index..index + frame_size,
            range_rel: 0..frame_size,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SliceInfo {
    pub size: usize,
    pub range_abs: Range<usize>,
    pub range_rel: Range<usize>,
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_info() -> Result<()> {
        let data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;

        assert_eq!(
            Info::try_from(&data[0..])?,
            Info {
                duration: 0,
                target: Target::All,
                command: Command::Read,
                header_size: 6,
                body_size: 450,
                total_size: 456,
            }
        );

        assert_eq!(
            Info::try_from(&data[456..])?,
            Info {
                duration: 0,
                target: Target::All,
                command: Command::Read,
                header_size: 6,
                body_size: 737,
                total_size: 743,
            }
        );

        Ok(())
    }

    #[test]
    fn test_detailed_info() -> Result<()> {
        let data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;

        assert_eq!(
            DetailedInfo::from_data_and_index(&data, 456)?,
            DetailedInfo {
                duration: 0,
                target: Target::All,
                command: Command::Read,
                header: SliceInfo {
                    size: 6,
                    range_abs: 456..462,
                    range_rel: 0..6,
                },
                body: SliceInfo {
                    size: 737,
                    range_abs: 462..1199,
                    range_rel: 6..743,
                },
                size: 743,
                range_abs: 456..1199,
                range_rel: 0..743,
            }
        );

        Ok(())
    }
}
