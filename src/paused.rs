use crate::validate;
use anyhow::Result;
use bstr::ByteSlice;
use std::io::{Read, Seek};

pub fn is_paused<R>(r: &mut R) -> Result<bool>
where
    R: Read + Seek,
{
    let data = r.bytes().collect::<Result<Vec<u8>, _>>()?;

    // Check if the demo has ended
    if validate::has_end_of_demo_marker(data.as_slice()) {
        return Ok(false);
    }

    // "Server is paused"
    const IS_PAUSED_NEEDLE: [u8; 0x10] = [
        0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x70, 0x61, 0x75, 0x73, 0x65,
        0x64,
    ];

    if data.rfind(IS_PAUSED_NEEDLE).is_some() {
        return Ok(true);
    }

    // "paused the game"
    const PAUSED_THE_GAME_NEEDLE: [u8; 0x0F] = [
        0x70, 0x61, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x61, 0x6D, 0x65,
    ];

    Ok(data.rfind(PAUSED_THE_GAME_NEEDLE).is_some())
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
