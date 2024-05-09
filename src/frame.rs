use std::io::Cursor;
use std::ops::Range;

use anyhow::Result;

use crate::mvd::io::ReadPrimitives;
use crate::numsize;
use crate::qw::{Command, Target};

pub const HEADER_SIZE: usize = numsize::SHORT + numsize::LONG;
pub const MULTI_HEADER_SIZE: usize = HEADER_SIZE + numsize::LONG;

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
    pub fn from_data_and_index(data: &[u8], index: usize) -> Result<Self> {
        let mut cur = Cursor::new(&data[index..]);

        let duration = cur.read_byte()? as u32;

        let (target, command) = {
            let byte = cur.read_byte()?;
            (Target::from(&byte), Command::from(&byte))
        };

        let multi_bytes = match target {
            Target::Multiple => numsize::LONG, // skip multi target bytes [0,0,0,0]
            _ => 0,
        };
        cur.set_position(cur.position() + multi_bytes as u64);

        let body_size = match command {
            Command::Read => cur.read_u32()? as usize,
            Command::Set => 2 * numsize::LONG, // reads 2 longs (8 bytes)
            _ => 0,                            // should not happen
        };

        let header_size = cur.position() as usize;
        let size = header_size + body_size;

        Ok(Self {
            index,
            duration,
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

        Ok(())
    }
}
