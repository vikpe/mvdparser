use crate::frame::Info;
use crate::message::Print;
use crate::qw::Message;

pub fn prints(data: &[u8]) -> Vec<Print> {
    let mut frame_offset = 0;
    let mut prints: Vec<Print> = Vec::new();

    while let Ok(frame_info) = Info::try_from(&data[frame_offset..]) {
        if frame_info.body_size > 0 {
            let msg_offset = frame_offset + frame_info.header_size;

            if Message::Print == Message::from(&data[msg_offset]) {
                let print_offset = msg_offset + 1;

                match Print::try_from(&data[print_offset..]) {
                    Ok(msg) => prints.push(msg),
                    Err(e) => println!("Error parsing print: {:?}", e),
                }
            }
        }

        frame_offset += frame_info.total_size;
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

        assert_eq!(2098, prints.len());
        assert_eq!(
            format!("{:?}", prints[0]),
            r#"Print { id: High, content: "bar.........axe is ready [oeks]_" }"#
        );
        assert_eq!(
            format!("{:?}", prints[1]),
            r#"Print { id: Chat, content: "Server starts recording (memory):_4on4_oeks_vs_tsq[dm2]20240426-1716.mvd_" }"#
        );

        Ok(())
    }
}
