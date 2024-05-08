use crate::frame;
use crate::message::Print;
use crate::qw::Message;

pub fn prints(data: &[u8]) -> Vec<Print> {
    let mut index = 0;
    let mut prints: Vec<Print> = Vec::new();

    while let Ok(frame_info) = frame::Info::from_data_and_index(data, index) {
        if frame_info.body_size > 0 {
            let msg_index = frame_info.header_range.end;

            if Message::Print == Message::from(&data[msg_index]) {
                let print_index = msg_index + 1;

                match Print::try_from(&data[print_index..]) {
                    Ok(msg) if !msg.content.is_empty() => prints.push(msg),
                    Err(e) => println!("Error parsing print: {:?}", e),
                    _ => {}
                }
            }
        }

        index += frame_info.size;
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

        assert_eq!(1225, prints.len());
        assert_eq!(
            format!("{:?}", prints[0]),
            r#"Print { id: High, content: "bar.........axe is ready [oeks]" }"#
        );
        assert_eq!(
            format!("{:?}", prints[1]),
            r#"Print { id: Chat, content: "Server starts recording (memory):_4on4_oeks_vs_tsq[dm2]20240426-1716.mvd" }"#
        );

        Ok(())
    }
}
