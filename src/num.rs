pub mod size {
    pub const SHORT: usize = 2;
    pub const LONG: usize = 4;
}

pub fn long(value: &[u8]) -> u32 {
    u32::from_le_bytes([value[0], value[1], value[2], value[3]])
}

pub fn short(value: &[u8]) -> u16 {
    u16::from_le_bytes([value[0], value[1]])
}
