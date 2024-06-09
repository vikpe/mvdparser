use std::fmt::Debug;
use std::io::{Cursor, Read};

use anyhow::Result;

use crate::qw::primitives::ReadPrimitives;
use crate::qw::prot::PrintId;

#[derive(PartialEq)]
pub struct Print {
    pub id: PrintId,
    pub content: Vec<u8>,
}

impl Print {
    pub fn byte_size(&self) -> usize {
        self.content.len() + 2 // id + null terminator
    }
}

impl Debug for Print {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Print")
            .field("id", &self.id)
            .field("content", &quake_text::bytestr::to_ascii(&self.content))
            .finish()
    }
}

impl TryFrom<&[u8]> for Print {
    type Error = std::io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Cursor::new(value).read_print()
    }
}

pub trait ReadPrint: ReadPrimitives {
    fn read_print(&mut self) -> std::io::Result<Print> {
        Ok(Print {
            id: PrintId::from(&self.read_byte()?),
            content: self.read_bstring()?,
        })
    }
}

impl<R: Read + ?Sized> ReadPrint for R {}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use crate::qw::message::Print;
    use crate::qw::prot::PrintId;

    #[test]
    fn test_try_from() -> Result<()> {
        // missing null terminator
        {
            let data: &[u8] = &[1, 2, 3, 4, 5, 10];
            assert_eq!(
                Print::try_from(data).unwrap_err().to_string(),
                "failed to read string".to_string()
            );
        }

        // valid
        {
            let data: &[u8] = &[1, 2, 3, 4, 10, 0];
            let print = Print {
                id: PrintId::Medium,
                content: vec![2, 3, 4, 10],
            };
            assert_eq!(Print::try_from(data)?, print);
            assert_eq!(print.byte_size(), 6);
        }

        Ok(())
    }
}
