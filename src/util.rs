use bstr::ByteSlice;

pub fn offsets_between(data: &[u8], a: &[u8], b: &[u8]) -> Option<(usize, usize)> {
    let index_from = data.find(a)? + a.len();
    let index_to = index_from + data[index_from..].find(b)?;

    if index_to > index_from {
        Some((index_from, index_to))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_offsets_between() -> Result<()> {
        let data = b"{'foo': 'bar', 'duration': 600, 'map': 'dm2'}";

        {
            let (from, to) = offsets_between(data, br#"'duration': "#, b",").unwrap();
            assert_eq!((from, to), (27, 30));
            assert_eq!(data[from..to].to_str()?, "600");
        }
        assert_eq!(offsets_between(data, b"", b","), Some((0, 13)));

        assert_eq!(offsets_between(data, br#"'map': "#, b","), None);
        assert_eq!(offsets_between(data, br#"'FOO': "#, b","), None);
        assert_eq!(offsets_between(data, br#"'duration': "#, b""), None);

        Ok(())
    }
}
