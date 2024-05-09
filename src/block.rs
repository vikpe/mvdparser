use std::io::Cursor;

use crate::mvd::io::ReadPrimitives;
use crate::numsize;
use crate::qw::HiddenMessage;

pub const HEADER_SIZE: usize = numsize::LONG + 2 * numsize::SHORT;

#[derive(Debug, PartialEq)]
pub struct Info {
    pub body_size: usize,
    pub total_size: usize,
    pub hidden_message: HiddenMessage,
    pub number: usize,
}

impl TryFrom<&[u8]> for Info {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut cur = Cursor::new(value);
        // exclude block number bytes from size
        let body_size = cur.read_u32()? as usize - numsize::SHORT;

        Ok(Info {
            body_size,
            total_size: HEADER_SIZE + body_size,
            hidden_message: HiddenMessage::from(&cur.read_u16()?),
            number: cur.read_u16()? as usize,
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
            Info::try_from(&data[13932032..])?,
            Info {
                body_size: 8084,
                total_size: 8092,
                hidden_message: HiddenMessage::Demoinfo,
                number: 1,
            }
        );

        assert_eq!(
            Info::try_from(&data[13940134..])?,
            Info {
                body_size: 6817,
                total_size: 6825,
                hidden_message: HiddenMessage::Demoinfo,
                number: 0,
            }
        );

        Ok(())
    }
}
