use anyhow::anyhow as e;
use std::io::{Read, Seek, SeekFrom};

pub fn last_n<R>(r: &mut R, n: usize) -> anyhow::Result<Vec<u8>>
where
    R: Read + Seek,
{
    let size = r.seek(SeekFrom::End(0))? as usize;

    if n > size {
        return Err(e!("n is larger than size"));
    }

    r.seek(SeekFrom::End(-(n as i64)))?;
    let mut buffer = vec![0; n];
    r.read_exact(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pkg::ioextra::last_n;
    use anyhow::Result;
    use std::io::Cursor;

    #[test]
    fn test_read_last_n_bytes() -> Result<()> {
        assert_eq!(last_n(&mut Cursor::new(b"Hello, World!"), 6)?, b"World!");
        assert!(last_n(&mut Cursor::new(b"Hello, World!"), 25).is_err());
        Ok(())
    }
}
