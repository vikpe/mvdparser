use std::io::{Read, Result};

use crate::mvd::io::ReadPrimitives;
use crate::qw;

pub trait ReadMessages: ReadPrimitives {
    fn read_message_type(&mut self) -> Result<qw::MessageType> {
        Ok(qw::MessageType::from(&self.read_byte()?))
    }
}

impl<R: Read + ?Sized> ReadMessages for R {}
