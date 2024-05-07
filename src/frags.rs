use std::collections::HashMap;

use bstr::BString;

use crate::{fragfile, prints};
use crate::qw::PrintId;

pub fn frags(data: &[u8]) -> HashMap<BString, i32> {
    let prints = prints::prints(data);

    for p in prints.iter().filter(|p| p.id == PrintId::Medium) {
        // println!("{:?}", quake_text::bytestr::to_ascii(&p.content));

        let asdasda = quake_text::bytestr::to_ascii(&p.content);
        match fragfile::Event::try_from(asdasda.as_str()) {
            Ok(event) => {
                println!("{:?}", event);
            }
            Err(e) => {
                println!("UNKNOWN {:?}", e);
            }
        }
    }

    HashMap::new()
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_frags() -> Result<()> {
        /*let demo_data = read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        let timer_start = std::time::Instant::now();
        let frags_map = frags(&demo_data);
        println!("took: {:?} ms", timer_start.elapsed().as_millis());
        let expected: HashMap<BString, i32> = HashMap::from([
            (BString::from("tco.........áøå"), 32),
            (BString::from("bar.........áøå"), 27),
            (BString::from("trl.........áøå"), 26),
            (BString::from("tim.........áøå"), 33),
            (BString::from("elguapo"), 60),
            (BString::from("conan"), 71),
            (BString::from("muttan"), 89),
            (BString::from("djevulsk"), 74),
        ]);
        assert_eq!(frags_map, expected);
*/
        Ok(())
    }
}
