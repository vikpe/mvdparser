use bstr::ByteSlice;
use quake_serverinfo::Serverinfo;

const NEEDLE: &[u8; 16] = br#"fullserverinfo ""#;

pub fn serverinfo(data: &[u8]) -> Option<Serverinfo> {
    serverinfo_string(data).map(|str| Serverinfo::from_str(&str))
}

pub fn serverinfo_string(data: &[u8]) -> Option<String> {
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

        let expected = Serverinfo {
            admin: Some("suom1 <suom1@irc.ax>".to_string()),
            deathmatch: Some(3),
            epoch: None,
            fpd: Some(142),
            fraglimit: None,
            gamedir: Some("qw".to_string()),
            hostname: Some("QUAKE.SE KTX:28501".to_string()),
            ktxmode: None,
            ktxver: Some("1.42".to_string()),
            map: Some("bravado".to_string()),
            matchtag: None,
            maxclients: Some(2),
            maxfps: Some(77),
            maxspectators: Some(12),
            mode: Some("1on1".to_string()),
            needpass: None,
            pm_ktjump: Some(1),
            progs: Some("so".to_string()),
            qvm: Some("so".to_string()),
            serverdemo: Some("duel_holy_vs_dago[bravado]20240426-1659.mvd".to_string()),
            status: Some("Countdown".to_string()),
            sv_antilag: Some(2),
            teamplay: None,
            timelimit: Some(10),
            version: Some("MVDSV 0.36".to_string()),
            z_ext: Some(511),
        };

        assert_eq!(serverinfo(&data), Some(expected));

        Ok(())
    }

    #[test]
    fn test_serverinfo_string() -> Result<()> {
        let data = std::fs::read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
        assert_eq!(
            serverinfo_string(&data),
            Some(
                r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\*admin\suom1 <suom1@irc.ax>\ktxver\1.42\sv_antilag\2\maxspectators\12\*gamedir\qw\timelimit\10\deathmatch\3\mode\1on1\hostname\QUAKE.SE KTX:28501\fpd\142\*qvm\so\*progs\so\maxclients\2\map\bravado\status\Countdown\serverdemo\duel_holy_vs_dago[bravado]20240426-1659.mvd"#.to_string()
            )
        );

        let data = std::fs::read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        assert_eq!(
            serverinfo_string(&data),
            Some(
                r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\*admin\suom1 <suom1@irc.ax>\ktxver\1.42\sv_antilag\2\maxspectators\12\teamplay\2\*gamedir\qw\maxclients\8\timelimit\20\deathmatch\1\mode\4on4\matchtag\tsq-axe lan\hostname\QUAKE.SE KTX:28502\fpd\142\*qvm\so\*progs\so\map\dm2\status\Countdown\serverdemo\4on4_oeks_vs_tsq[dm2]20240426-1716.mvd"#.to_string()
            )
        );

        Ok(())
    }
}
