use std::fmt::Debug;

use anyhow::{anyhow as e, Error};
use bstr::ByteSlice;

use crate::qw::PrintId;

mod index {
    pub const ID: usize = 0;
    pub const CONTENT_FROM: usize = 1;
}

#[derive(PartialEq)]
pub struct Print {
    pub id: PrintId,
    pub content: Vec<u8>,
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
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(e!("Insufficient length"));
        }

        let Some(content_index_to) = value.find([0]) else {
            return Err(e!("Missing null terminator"));
        };

        let content = match content_index_to > index::CONTENT_FROM {
            true => &value[index::CONTENT_FROM..content_index_to - 1], // skip \n
            false => &[],
        }
        .to_vec();

        Ok(Self {
            id: PrintId::from(&value[index::ID]),
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use crate::message::print::Print;
    use crate::qw::PrintId;

    #[test]
    fn test_try_from() -> Result<()> {
        // invalid length
        {
            let data: &[u8] = &[1, 0];
            assert_eq!(
                Print::try_from(data).unwrap_err().to_string(),
                "Insufficient length".to_string()
            );
        }

        // missing null terminator
        {
            let data: &[u8] = &[1, 2, 3, 4, 5];
            assert_eq!(
                Print::try_from(data).unwrap_err().to_string(),
                "Missing null terminator".to_string()
            );
        }

        // valid
        {
            let data: &[u8] = &[1, 2, 3, 4, 10, 0];
            assert_eq!(
                Print::try_from(data)?,
                Print {
                    id: PrintId::Medium,
                    content: vec![2, 3, 4],
                }
            );
        }

        Ok(())
    }
}
