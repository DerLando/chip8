const CHIP8_START: usize = 0x200;
const ETI660_START: usize = 0x200;

pub(crate) struct Memory {
    buffer: [u8; 4096],
}

impl Memory {
    pub(crate) fn load(&self, ptr: u16) -> u16 {
        let ptr = ptr as usize;
        u16::from_be_bytes(
            self.buffer[ptr..=ptr + 1]
                .try_into()
                .expect("Buffer big enough"),
        )
    }
}

pub(crate) struct Stack {
    buffer: [u16; 16],
}
