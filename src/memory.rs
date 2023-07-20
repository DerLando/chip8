pub(crate) const CHIP8_START: usize = 0x200;
const ETI660_START: usize = 0x200;

pub(crate) struct Memory {
    buffer: [u8; 4096],
}

impl Memory {
    pub(crate) fn new() -> Self {
        Self { buffer: [0; 4096] }
    }

    pub(crate) fn load(&self, ptr: u16) -> u16 {
        let ptr = ptr as usize;
        u16::from_be_bytes(
            self.buffer[ptr..=ptr + 1]
                .try_into()
                .expect("Buffer big enough"),
        )
    }

    pub(crate) fn store(&mut self, ptr: u16, value: u16) {
        let ptr = ptr as usize;
        let values = value.to_be_bytes();
        self.buffer[ptr] = values[0];
        self.buffer[ptr + 1] = values[1];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_load_store() {
        let mut memory = Memory::new();
        memory.store(2, 0x200);
        assert_eq!(0x200, memory.load(2));
    }
}

pub(crate) struct Stack {
    ptr: usize,
    buffer: [u16; 16],
}

impl Stack {
    pub fn new() -> Self {
        Self {
            ptr: 0,
            buffer: [0; 16],
        }
    }

    pub fn push(&mut self, value: u16) {
        self.buffer[self.ptr] = value;
        self.ptr += 1;
    }
    pub fn pop(&mut self) -> u16 {
        let value = self.buffer[self.ptr];
        self.ptr -= 1;
        value
    }
}
