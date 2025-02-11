use crate::pkg::ioextra;
use anyhow::Result;
use std::io::{Read, Seek};

pub fn has_end_of_demo_marker<R>(r: &mut R) -> bool
where
    R: Read + Seek,
{
    // "[print] EndOfDemo"
    const NEEDLE: [u8; 12] = [0, 2, 69, 110, 100, 79, 102, 68, 101, 109, 111, 0];
    const NLEN: usize = NEEDLE.len();

    match ioextra::last_n(r, NLEN) {
        Ok(suffix) => suffix == NEEDLE,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_has_end_of_demo_marker() -> Result<()> {
        assert!(has_end_of_demo_marker(&mut File::open(
            "tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd"
        )?));
        assert!(has_end_of_demo_marker(&mut File::open(
            "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
        )?));
        assert!(!has_end_of_demo_marker(&mut File::open(
            "tests/files/4on4_-s-_vs_pol[dm2]20241118-2135.mvd"
        )?));

        Ok(())
    }
}
