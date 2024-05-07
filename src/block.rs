use anyhow::anyhow as e;

use crate::num;
use crate::qw::HiddenMessage;

pub const HEADER_SIZE: usize = num::size::LONG + 2 * num::size::SHORT;

mod index {
    pub const SIZE: usize = 0;
    pub const HIDDEN_MESSAGE: usize = 4;
    pub const NUMBER: usize = 6;
}

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
        if value.len() < HEADER_SIZE {
            return Err(e!("block::Info: insufficient length"));
        }

        // exclude block number bytes from size
        let body_size = num::long(&value[index::SIZE..]) as usize - num::size::SHORT;

        Ok(Info {
            body_size,
            total_size: HEADER_SIZE + body_size,
            hidden_message: HiddenMessage::from(num::short(&value[index::HIDDEN_MESSAGE..])),
            number: num::short(&value[index::NUMBER..]) as usize,
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
