const CHIP8_START: usize = 0x200;
const ETI660_START: usize = 0x200;

pub(crate) struct Memory {
    buffer: [u8; 4096],
}

impl Memory {}
