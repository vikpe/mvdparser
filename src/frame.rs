use anyhow::anyhow as e;

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

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_frameinfo() -> Result<()> {
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
}
