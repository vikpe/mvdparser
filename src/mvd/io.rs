use std::io::{Error, ErrorKind, Read, Result};

pub fn err_other(msg: &str) -> Error {
    Error::new(ErrorKind::Other, msg)
}

pub trait ReadPrimitives: Read {
    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)
            .map_err(|_| err_other("failed to read byte"))?;
        Ok(buf[0])
    }

    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)
            .map_err(|_| err_other("failed to read short"))?;
        Ok(u16::from_le_bytes([buf[0], buf[1]]))
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)
            .map_err(|_| err_other("failed to read long"))?;
        Ok(u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]))
    }

    fn read_bstring(&mut self) -> Result<Vec<u8>> {
        let mut result = Vec::new();
        let mut buf = [0; 1];
        loop {
            self.read_exact(&mut buf)
                .map_err(|_| err_other("failed to read string"))?;
            if buf == [0] {
                break;
            }
            result.push(buf[0]);
        }
        Ok(result)
    }
}

impl<R: Read + ?Sized> ReadPrimitives for R {}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_read_byte() {
        let data: &[u8] = &[8];
        let mut buf = Cursor::new(data);
        assert_eq!(buf.read_byte().unwrap(), 8);
    }

    #[test]
    fn test_read_u16() {
        let data: &[u8] = &[1, 2];
        let mut buf = Cursor::new(data);
        assert_eq!(buf.read_u16().unwrap(), 513);
    }

    #[test]
    fn test_read_u32() {
        let data: &[u8] = &[1, 2, 3, 1];
        let mut buf = Cursor::new(data);
        assert_eq!(buf.read_u32().unwrap(), 16974337);
    }

    #[test]
    fn test_read_bstring() {
        let data: &[u8] = &[1, 2, 3, 10, 0];
        let mut buf = Cursor::new(data);
        assert_eq!(buf.read_bstring().unwrap(), vec![1, 2, 3, 10]);
    }
}
