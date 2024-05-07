use std::time::Duration;

use bstr::ByteSlice;

use crate::frame;
use crate::frame::Info;
use crate::qw::Message;

pub fn is_player_msg(msg: &str) -> bool {
    let needles = [
        "tco", "bar", "trl", "tim", "elguapo", "conan", "muttan", "djevulsk",
    ];
    needles.iter().any(|name| msg.contains(name))
}

pub fn is_negative_frag(msg: &str) -> bool {
    let needles = [
        " becomes bored with life",
        " discovers blast radius",
        " gets a frag for the other team",
        " mows down a teammate",
        " loses another friend",
        " tries to put the pin back in",
        " was telefragged ",
        " ate a lavaball",
        " blew up",
        " burst into flames",
        " can't exist on slime alone",
        " checks ",
        " cratered",
        " died",
        " discharges ",
        " electrocutes ",
        " fell to ",
        " gulped a load of slime",
        " heats up the water",
        " railcutes ",
        " sleeps with the fishes",
        " squished a teammate",
        " stomps ",
        " sucks it down",
        " suicides",
        " tried to catch ",
        " tried to leave",
        " turned into hot slag",
        " visits the Volcano God",
        " was crushed ",
        " was jumped ",
        " was spiked",
        " was squished",
        " was zapped",
        r#" softens "   "'s fall"#,
    ];
    needles.iter().any(|name| msg.contains(name))
}

pub fn frag_diff(msg: &str) -> i32 {
    let nei = [": ", "dropped", "is ready"];
    if nei.iter().any(|name| msg.contains(name)) {
        return 0;
    }

    if is_player_msg(msg) {
        if is_negative_frag(msg) {
            return -1;
        }

        return 1;
    }

    0
}

pub fn prints(data: &[u8]) {
    let now = std::time::Instant::now();
    let mut offset = 0;
    let mut frag_count: i32 = 0;
    let mut total_ms = 0;

    while (offset + frame::MULTI_HEADER_SIZE) < data.len() {
        let Ok(frame_info) = Info::try_from(&data[offset..offset + frame::MULTI_HEADER_SIZE])
        else {
            println!(
                "### {}s - FrameInfo: invalid length",
                total_ms as f64 / 1000.0
            );
            break;
        };

        total_ms += frame_info.duration;

        if frame_info.body_size == 0 {
            offset += frame_info.total_size;
            continue;
        }

        let msg_type_byte = data[offset + frame_info.header_size];
        let msg_type = Message::from(msg_type_byte);

        if Message::Unknown == msg_type {
            println!(
                "#{} {}s - msg: {} - size: {}",
                offset,
                (total_ms as f64 / 1000.0).round(),
                msg_type_byte,
                frame_info.body_size
            );
            println!("### {:?}", quake_text::bytestr::to_ascii(&data[offset..offset + frame_info.total_size]));
            break;
        }

        if Message::Print == msg_type {
            let pfrom = offset + 8;

            if data[pfrom] != 40 {
                // opening brace = tp msg
                let pto = data[pfrom..].find([0]).map(|o| o + pfrom).unwrap();

                if pto > pfrom {
                    let content = quake_text::bytestr::to_ascii(&data[pfrom..pto - 1]);

                    if is_player_msg(&content) {
                        let diff = frag_diff(&content);

                        if diff != 0 {
                            frag_count += diff;
                            let duration =
                                Duration::from_secs_f64((total_ms as f64 / 1000.0) - 10.0);

                            let duration_hhss = format!(
                                "{:02}:{:02}",
                                duration.as_secs() / 60,
                                duration.as_secs() % 60
                            );

                            println!(
                                "### {}  {:>4} {:>2} - {}",
                                duration_hhss, frag_count, diff, content,
                            );
                        }
                    }
                }
            }
        }

        offset += frame_info.total_size;
    }

    println!();
    println!("### duration: {}ms", now.elapsed().as_millis());
    println!("### frags: {}", frag_count);
    println!();
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;

    use super::*;

    #[test]
    fn test_prints() -> Result<()> {
        let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
        prints(&demo_data);
        panic!();
        Ok(())
    }
}
