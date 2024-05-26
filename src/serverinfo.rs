use quake_serverinfo::Serverinfo;

use crate::util;

pub fn serverinfo(data: &[u8]) -> Option<Serverinfo> {
    serverinfo_string(data).map(|str| Serverinfo::from(str.as_str()))
}

pub fn serverinfo_string(data: &[u8]) -> Option<String> {
    const MAX_OFFSET: usize = 128;
    const MAX_SIZE: usize = 1024;
    let (from, to) = util::offsets_between(
        &data[..MAX_OFFSET + MAX_SIZE],
        br#"fullserverinfo ""#,
        &[b'"'],
    )?;
    String::from_utf8(data[from..to].to_vec()).ok()
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_serverinfo() -> Result<()> {
        let data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;

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
        assert_eq!(
            serverinfo_string(&read("tests/files/duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd")?),
            Some(
                r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 1.01-dev\*z_ext\511\maxspectators\12\*gamedir\qw\sv_antilag\2\*admin\ERRH @ https://discord.quake.world\ktxver\1.44-dev\mode\1on1\maxclients\2\timelimit\3\deathmatch\4\hostname\de.quake.world:27502 [QW-Group]\fpd\142\*qvm\so\*progs\so\map\povdmm4\status\Countdown\serverdemo\duel_equ_vs_kaboom[povdmm4]20240422-1038.mvd\epoch\1713782300"#.to_string()
            )
        );

        assert_eq!(
            serverinfo_string(&read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?),
            Some(
                r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\*admin\suom1 <suom1@irc.ax>\ktxver\1.42\sv_antilag\2\maxspectators\12\*gamedir\qw\timelimit\10\deathmatch\3\mode\1on1\hostname\QUAKE.SE KTX:28501\fpd\142\*qvm\so\*progs\so\maxclients\2\map\bravado\status\Countdown\serverdemo\duel_holy_vs_dago[bravado]20240426-1659.mvd"#.to_string()
            )
        );

        assert_eq!(
            serverinfo_string(&read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?),
            Some(
                r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\*admin\suom1 <suom1@irc.ax>\ktxver\1.42\sv_antilag\2\maxspectators\12\teamplay\2\*gamedir\qw\maxclients\8\timelimit\20\deathmatch\1\mode\4on4\matchtag\tsq-axe lan\hostname\QUAKE.SE KTX:28502\fpd\142\*qvm\so\*progs\so\map\dm2\status\Countdown\serverdemo\4on4_oeks_vs_tsq[dm2]20240426-1716.mvd"#.to_string()
            )
        );

        assert_eq!(
            serverinfo_string(&read("tests/files/ctf_blue_vs_red[ctf5]20240520-1925.mvd")?),
            Some(
                r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 1.01-dev\*z_ext\511\maxspectators\12\*admin\QuakeWorld CTF Discord <tag@discord>\ktxver\1.43-dev\sv_antilag\2\mode\ctf\*gamedir\qw\teamplay\4\deathmatch\3\fpd\142\maxclients\16\watervis\1\timelimit\10\hostname\qwctf.se:28501\*qvm\so\*progs\so\map\ctf5\status\Countdown\serverdemo\ctf_blue_vs_red[ctf5]20240520-1925.mvd\epoch\1716233132"#.to_string()
            )
        );

        Ok(())
    }
}

pub mod analyze {
    use quake_serverinfo::Serverinfo;

    pub fn is_ctf(info: &Serverinfo) -> bool {
        [
            info.mode.as_ref().is_some_and(|mode| mode == "ctf"),
            info.ktxmode.as_ref().is_some_and(|mode| mode == "ctf"),
            info.teamplay == Some(4) && info.deathmatch == Some(3),
            info.serverdemo
                .as_ref()
                .is_some_and(|demo| demo.starts_with("ctf_")),
        ]
        .iter()
        .any(|c| *c)
    }

    #[cfg(test)]
    mod tests {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn test_is_ctf() {
            assert_eq!(is_ctf(&Serverinfo::from(r#"\mode\ctf"#)), true);
            assert_eq!(is_ctf(&Serverinfo::from(r#"\ktxmode\ctf"#)), true);
            assert_eq!(
                is_ctf(&Serverinfo::from(r#"\teamplay\4\deathmatch\3"#)),
                true
            );
            assert_eq!(
                is_ctf(&Serverinfo::from(
                    r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\*admin\suom1 <suom1@irc.ax>\ktxver\1.42\sv_antilag\2\maxspectators\12\teamplay\2\*gamedir\qw\maxclients\8\timelimit\20\deathmatch\1\mode\4on4\matchtag\tsq-axe lan\hostname\QUAKE.SE KTX:28502\fpd\142\*qvm\so\*progs\so\map\dm2\status\Countdown\serverdemo\4on4_oeks_vs_tsq[dm2]20240426-1716.mvd"#
                )),
                false
            );
        }
    }
}
