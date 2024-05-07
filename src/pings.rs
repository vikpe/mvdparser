const P_PING: usize = 14;

fn is_ping_update(w: &[u8]) -> bool {
    w[1] == 36 && w[2] == 0 && w[5] == 53 && w[8] == 36 && w[9] == 1 && w[12] == 53 && w[13] == 1
}

fn ping_updates(data: &[u8]) {
    let now = std::time::Instant::now();

    let mut windows = data.windows(P_PING);
    let mut offset = data.len();

    let max_samples = 10;
    let mut sample_count = 0;

    while let Some(w) = windows.next_back() {
        if w.len() < P_PING {
            break;
        }

        if is_ping_update(w) {
            println!("#{}: {:?}", offset, w);
            sample_count += 1;
        }

        if sample_count >= max_samples {
            break;
        }

        offset -= 1;
    }

    println!("total duration: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;

    use super::*;

    #[test]
    fn test_countdown_duration() -> Result<()> {
        let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        ping_updates(&demo_data);
        panic!();

        Ok(())
    }
}
