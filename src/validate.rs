pub fn has_end_of_demo_marker(data: &[u8]) -> bool {
    const NEEDLE: [u8; 12] = [
        0x00, 0x02, 0x45, 0x6E, 0x64, 0x4F, 0x66, 0x44, 0x65, 0x6D, 0x6F, 0x00,
    ]; // "[print] EndOfDemo"
    const NEEDLE_LEN: usize = NEEDLE.len();
    data.len() > NEEDLE_LEN && data[data.len() - NEEDLE_LEN..] == NEEDLE
}
