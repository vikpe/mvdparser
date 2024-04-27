pub const DELIMITER: &[u8; 4] = b"\x00\x03\x00\x00";

pub mod info {
    pub const LENGTH: usize = 18;
    pub const LENGTH_INDEX: usize = 10;
}

pub fn length(data: &[u8], offset: usize) -> usize {
    let index = offset + info::LENGTH_INDEX;
    u16::from_le_bytes([data[index], data[index + 1]]) as usize - 2
}
