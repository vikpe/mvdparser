use bstr::ByteSlice;

const NEEDLE: &[u8; 16] = br#"fullserverinfo ""#;

pub fn serverinfo(data: &[u8]) -> Option<String> {
    let index_from = data.find(NEEDLE)? + NEEDLE.len();
    let index_to = index_from + data[index_from..].find_byte(b'"')?;
    String::from_utf8(data[index_from..index_to].to_vec()).ok()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_serverinfo() -> Result<()> {
        let data = std::fs::read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
        assert_eq!(
            serverinfo(&data),
            Some(
                r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\*admin\suom1 <suom1@irc.ax>\ktxver\1.42\sv_antilag\2\maxspectators\12\*gamedir\qw\timelimit\10\deathmatch\3\mode\1on1\hostname\QUAKE.SE KTX:28501\fpd\142\*qvm\so\*progs\so\maxclients\2\map\bravado\status\Countdown\serverdemo\duel_holy_vs_dago[bravado]20240426-1659.mvd"#.to_string()
            )
        );

        let data = std::fs::read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        assert_eq!(
            serverinfo(&data),
            Some(
                r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\*admin\suom1 <suom1@irc.ax>\ktxver\1.42\sv_antilag\2\maxspectators\12\teamplay\2\*gamedir\qw\maxclients\8\timelimit\20\deathmatch\1\mode\4on4\matchtag\tsq-axe lan\hostname\QUAKE.SE KTX:28502\fpd\142\*qvm\so\*progs\so\map\dm2\status\Countdown\serverdemo\4on4_oeks_vs_tsq[dm2]20240426-1716.mvd"#.to_string()
            )
        );

        Ok(())
    }
}
