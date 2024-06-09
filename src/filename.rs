use anyhow::{anyhow as e, Result};

use crate::serverinfo;

pub fn filename(data: &[u8]) -> Result<String> {
    let Some(serverdemo) = serverinfo(data)?.serverdemo else {
        return Err(e!("Filename not found"));
    };
    Ok(serverdemo)
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_filename() -> Result<()> {
        assert_eq!(
            filename(&read(
                "tests/files/2on2_sf_vs_red[frobodm2]220104-0915.mvd"
            )?)?,
            "2on2_sf_vs_red[frobodm2]220104-0915.mvd".to_string()
        );

        assert_eq!(
            filename(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?)?,
            "4on4_oeks_vs_tsq[dm2]20240426-1716.mvd".to_string()
        );

        assert_eq!(
            filename(&read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?)?,
            "ctf_blue_vs_red[ctf5]20240520-1925.mvd".to_string()
        );

        assert_eq!(
            filename(&read(
                "tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd"
            )?)?,
            "duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd".to_string()
        );

        assert_eq!(
            filename(&read(
                "tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd"
            )?)?,
            "duel_holy_vs_dago[bravado]20240426-1659.mvd".to_string()
        );

        assert_eq!(
            filename(&read("tests/files/ffa_5[dm4]20240501-1229.mvd")?)?,
            "ffa_5[dm4]20240501-1229.mvd".to_string()
        );

        assert_eq!(
            filename(&read(
                "tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd"
            )?)?,
            "wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd".to_string()
        );

        Ok(())
    }
}
