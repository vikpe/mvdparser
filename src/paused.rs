use crate::pkg::ioextra::last_n;
use crate::validate::has_end_of_demo_marker;
use anyhow::Result;
use bstr::ByteSlice;
use std::io::{Read, Seek};

// "Server is paused"
const NEEDLE_1: [u8; 16] = [
    83, 101, 114, 118, 101, 114, 32, 105, 115, 32, 112, 97, 117, 115, 101, 100,
];

// "paused the game"
const NEEDLE_2: [u8; 15] = [
    112, 97, 117, 115, 101, 100, 32, 116, 104, 101, 32, 103, 97, 109, 101,
];

pub fn is_paused<R>(r: &mut R) -> Result<bool>
where
    R: Read + Seek,
{
    if has_end_of_demo_marker(r) {
        return Ok(false);
    }

    let last_bytes = last_n(r, 2048)?;
    Ok(last_bytes.rfind(NEEDLE_1).is_some() || last_bytes.rfind(NEEDLE_2).is_some())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::fs::File;

    use super::*;

    #[test]
    fn test_is_paused() -> Result<()> {
        assert!(!is_paused(&mut File::open(
            "tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd"
        )?)?);
        assert!(!is_paused(&mut File::open(
            "tests/files/ffa_5[dm4]20240501-1229.mvd"
        )?)?);
        assert!(is_paused(&mut File::open(
            "tests/files/4on4_-s-_vs_pol[dm2]20241118-2135.mvd"
        )?)?);
        assert!(!is_paused(&mut File::open(
            "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd"
        )?)?);
        assert!(!is_paused(&mut File::open(
            "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
        )?)?);
        assert!(!is_paused(&mut File::open(
            "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
        )?)?);
        assert!(!is_paused(&mut File::open(
            "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd"
        )?)?);
        assert!(!is_paused(&mut File::open(
            "tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd"
        )?)?);

        Ok(())
    }
}
