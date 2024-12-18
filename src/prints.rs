use std::io::Cursor;

use crate::qw::frame;
use crate::qw::message::message_type::ReadMessageType;
use crate::qw::message::print::ReadPrint;
use crate::qw::message::Print;
use crate::qw::prot::MessageType;

pub fn prints(data: &[u8]) -> Vec<Print> {
    let mut index = 0;
    let mut prints: Vec<Print> = Vec::new();

    while let Ok(info) = frame::Info::from_data_and_index(data, index) {
        if info.body_size > 0 {
            let mut body = Cursor::new(&data[info.body_range]);

            while body
                .read_message_type()
                .is_ok_and(|t| t == MessageType::Print)
            {
                if let Ok(print) = body.read_print() {
                    if !print.content.is_empty() {
                        prints.push(print);
                    }
                }
            }
        }

        index += info.size;
    }

    prints.dedup();
    prints
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_prints() -> Result<()> {
        let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        let prints = prints(&demo_data);

        assert_eq!(1273, prints.len());
        assert_eq!(
            format!("{:?}", prints[0]),
            r#"Print { id: High, content: "bar.........axe is ready [oeks]_" }"#
        );
        assert_eq!(
            format!("{:?}", prints[1]),
            r#"Print { id: High, content: "All players ready_" }"#
        );
        assert_eq!(
            format!("{:?}", prints[2]),
            r#"Print { id: High, content: "Timer started_" }"#
        );

        Ok(())
    }
}
