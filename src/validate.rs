use crate::{clientinfo, serverinfo};

pub fn is_valid(data: &[u8]) -> bool {
    const MIN_SIZE: usize = 1024;
    (data.len() >= MIN_SIZE)
        && has_end_of_demo_print(data)
        && serverinfo(data).is_ok()
        && has_clients(data)
}

fn has_clients(data: &[u8]) -> bool {
    clientinfo(data).is_ok_and(|c| !c.is_empty())
}

fn has_end_of_demo_print(data: &[u8]) -> bool {
    const NEEDLE: [u8; 12] = [
        0x00, 0x02, 0x45, 0x6E, 0x64, 0x4F, 0x66, 0x44, 0x65, 0x6D, 0x6F, 0x00,
    ]; // "[print] EndOfDemo"
    const NEEDLE_LEN: usize = NEEDLE.len();
    data.len() > NEEDLE_LEN && data[data.len() - NEEDLE_LEN..] == NEEDLE
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;

    use super::*;

    #[test]
    fn test_is_valid() -> Result<()> {
        assert!(!is_valid(&[]));
        assert!(!is_valid(&[10; 128]));
        assert!(!is_valid(&[10; 1280]));

        let demo_paths = vec![
            "tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd",
            "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd",
            "tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd",
            "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd",
            "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd",
            "tests/files/ffa_5[dm4]20240501-1229.mvd",
            "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd",
        ];

        for path in demo_paths {
            let demo_data = read(path)?;
            assert!(is_valid(&demo_data), "{}", path.to_string());
        }

        // no serverinfo
        {
            let demo_data = read("tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd")?;
            assert!(!is_valid(&demo_data[2000..]));
        }

        Ok(())
    }

    #[test]
    fn test_has_clients() -> Result<()> {
        let demo_data = read("tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd")?;
        assert!(has_clients(&demo_data));
        assert!(!has_clients(&demo_data[5000..]));
        Ok(())
    }

    #[test]
    fn test_is_has_end_of_demo_print() -> Result<()> {
        let demo_data = read("tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd")?;
        assert!(has_end_of_demo_print(&demo_data));
        assert!(!has_end_of_demo_print(&demo_data[..demo_data.len() - 32]));
        Ok(())
    }
}
