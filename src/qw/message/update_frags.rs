use std::io::{Cursor, Read};

use crate::qw::primitives::ReadPrimitives;

#[derive(Debug, PartialEq)]
pub struct UpdateFrags {
    pub player_number: u8,
    pub frags: u16,
}

impl TryFrom<&[u8]> for UpdateFrags {
    type Error = std::io::Error;

    fn try_from(value: &[u8]) -> anyhow::Result<Self, Self::Error> {
        Cursor::new(value).read_update_frags()
    }
}

pub trait ReadUpdateFrags: ReadPrimitives {
    fn read_update_frags(&mut self) -> std::io::Result<UpdateFrags> {
        Ok(UpdateFrags {
            player_number: self.read_byte()?,
            frags: self.read_u16()?,
        })
    }
}

impl<R: Read + ?Sized> ReadUpdateFrags for R {}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_update_frags() {
        let bytes: &[u8] = &[9, 23, 1];

        assert_eq!(
            UpdateFrags::try_from(bytes).unwrap(),
            UpdateFrags {
                player_number: 9,
                frags: 279,
            }
        );
    }
}
