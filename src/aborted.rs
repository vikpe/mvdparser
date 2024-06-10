use bstr::ByteSlice;

const MATCH_STOPPED_PRINT: [u8; 0x22] = [
    0x08, 0x02, 0xCD, 0xE1, 0xF4, 0xE3, 0xE8, 0x20, 0xF3, 0xF4, 0xEF, 0xF0, 0xF0, 0xE5, 0xE4, 0x20,
    0xE2, 0xF9, 0x20, 0xED, 0xE1, 0xEA, 0xEF, 0xF2, 0xE9, 0xF4, 0xF9, 0x20, 0xF6, 0xEF, 0xF4, 0xE5,
    0x0A, 0x00,
]; // "[print] Match stopped by majority vote"

pub fn is_aborted(data: &[u8]) -> bool {
    data.find(MATCH_STOPPED_PRINT).is_some()
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;

    use super::*;

    #[test]
    fn test_is_aborted() -> Result<()> {
        assert!(is_aborted(&read(
            "tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd"
        )?));
        assert!(is_aborted(&read(
            "tests/files/ffa_5[dm4]20240501-1229.mvd"
        )?));
        assert!(!is_aborted(&read(
            "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd"
        )?));
        assert!(!is_aborted(&read(
            "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
        )?));
        assert!(!is_aborted(&read(
            "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
        )?));
        assert!(!is_aborted(&read(
            "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd"
        )?));
        assert!(!is_aborted(&read(
            "tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd"
        )?));

        Ok(())
    }
}
