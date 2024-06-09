use std::io::{Read, Result};

use crate::qw::primitives::ReadPrimitives;
use crate::qw::prot::MessageType;

pub trait ReadMessageType: ReadPrimitives {
    fn read_message_type(&mut self) -> Result<MessageType> {
        Ok(MessageType::from(&self.read_byte()?))
    }
}

impl<R: Read + ?Sized> ReadMessageType for R {}
