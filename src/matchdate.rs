use bstr::ByteSlice;

const NEEDLE: &[u8; 11] = b"matchdate: ";
const MIN_LEN: usize = "yyyy-mm-dd hh:mm:ss ab".len();
const MAX_LEN: usize = "yyyy-mm-dd hh:mm:ss abcde".len();

pub fn matchdate(data: &[u8]) -> Option<String> {
    let needle_index = data.find(NEEDLE)?;
    let index_from = needle_index + NEEDLE.len();
    let index_to = data[needle_index..].find_byte(b'\n')?;
    let length = index_to - index_from;

    if !(MIN_LEN..=MAX_LEN).contains(&length) {
        return None;
    }

    String::from_utf8(data[index_from..index_to].to_vec()).ok()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_matchdate() -> Result<()> {
        assert_eq!(matchdate(b""), None);
        assert_eq!(matchdate(b"foo"), None);
        assert_eq!(matchdate(b"matchdate: foo"), None);
        assert_eq!(matchdate(b"matchdate: 2024"), None);

        let data = std::fs::read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
        assert_eq!(
            matchdate(&data),
            Some("2024-04-26 16:59:29 CEST".to_string())
        );

        let data = std::fs::read("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd")?;
        assert_eq!(
            matchdate(&data),
            Some("2024-04-26 17:16:13 CEST".to_string()),
        );
        Ok(())
    }
}
