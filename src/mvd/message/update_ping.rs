use std::io::{Cursor, Read};

use crate::mvd::io::ReadPrimitives;

#[derive(Debug, PartialEq)]
pub struct UpdatePing {
    pub player_number: u8,
    pub ping: u16,
}

impl TryFrom<&[u8]> for UpdatePing {
    type Error = std::io::Error;

    fn try_from(value: &[u8]) -> anyhow::Result<Self, Self::Error> {
        Cursor::new(value).read_update_ping()
    }
}

pub trait ReadUpdatePing: ReadPrimitives {
    fn read_update_ping(&mut self) -> std::io::Result<UpdatePing> {
        let ping = UpdatePing {
            player_number: self.read_byte()?,
            ping: self.read_u16()?,
        };
        self.read_byte()?;
        Ok(ping)
    }
}

impl<R: Read + ?Sized> ReadUpdatePing for R {}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::mvd::message::UpdatePing;

    #[test]
    fn test_update_ping() {
        let bytes: &[u8] = &[4, 1, 2, 0];

        assert_eq!(
            UpdatePing::try_from(bytes).unwrap(),
            UpdatePing {
                player_number: 4,
                ping: 513,
            }
        );
    }
}
