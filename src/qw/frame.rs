use crate::qw::hidden_message::HiddenMessage;
use binrw::BinRead;

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct Frame {
    pub duration: u8,
    #[br(map = |x: u8| Command::from(x))]
    pub command: Command,
    #[br(if(command == Command::Multiple, None))]
    pub target: Option<u32>,
    pub body_size: u32,
}

impl Frame {
    pub fn is_empty(&self) -> bool {
        self.body_size == 0
    }
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MultiFrameInfo {
    pub body_size: u32,
    pub hidden_message: HiddenMessage,
}

#[derive(Clone, Debug, PartialEq, BinRead)]
#[br(repr=u8)]
pub enum Command {
    Cmd = 0,      // A user cmd movement message.
    Read = 1,     // A net message.
    Set = 2,      // Appears only once at the beginning of a demo
    Multiple = 3, // MVD ONLY. This message is directed to several clients.
    Single = 4,   // MVD ONLY. This message is directed to a single client.
    Stats = 5,    // MVD ONLY. Stats update for a player.
    All = 6,      // MVD ONLY. This message is directed to all clients.
    Empty = 7,    //
}

impl From<u8> for Command {
    fn from(value: u8) -> Self {
        // read 3 first bytes
        match value & 7 {
            0 => Command::Cmd,
            1 => Command::Read,
            2 => Command::Set,
            3 => Command::Multiple,
            4 => Command::Single,
            5 => Command::Stats,
            6 => Command::All,
            _ => Command::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    #[test]
    fn test_frame_read() -> Result<()> {
        {
            // All
            let info = Frame::read(&mut Cursor::new([0, 6, 232, 1, 0, 0]))?;
            assert_eq!(info.duration, 0);
            assert_eq!(info.command, Command::All);
            assert_eq!(info.target, None);
            assert_eq!(info.body_size, 488);
        }
        {
            // Multiple
            let info = Frame::read(&mut Cursor::new([0, 3, 0, 0, 0, 0, 232, 1, 0, 0]))?;
            assert_eq!(info.duration, 0);
            assert_eq!(info.command, Command::Multiple);
            assert_eq!(info.target, Some(0));
            assert_eq!(info.body_size, 488);
        }

        Ok(())
    }

    #[test]
    fn test_command_from() -> Result<()> {
        assert_eq!(Command::from(0), Command::Cmd);
        assert_eq!(Command::from(7), Command::Empty);
        assert_eq!(Command::from(8), Command::Cmd);
        Ok(())
    }
}
