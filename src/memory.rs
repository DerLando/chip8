pub(crate) const CHIP8_START: usize = 0x200;
pub(crate) const MEMORY_SIZE: usize = 4096;
const ETI660_START: usize = 0x200;

pub(crate) struct Memory {
    buffer: [u8; MEMORY_SIZE],
}

impl Memory {
    pub(crate) fn new() -> Self {
        Self {
            buffer: [0; MEMORY_SIZE],
        }
    }

    pub(crate) fn read_u16(&self, ptr: u16) -> u16 {
        let ptr = ptr as usize;
        u16::from_be_bytes(
            self.buffer[ptr..=ptr + 1]
                .try_into()
                .expect("Buffer big enough"),
        )
    }

    pub(crate) fn clear_public(&mut self) {
        self.buffer[CHIP8_START..MEMORY_SIZE].copy_from_slice(&[0; MEMORY_SIZE - CHIP8_START]);
    }

    pub(crate) fn read_u8(&self, ptr: u16) -> u8 {
        self.buffer[ptr as usize]
    }

    pub(crate) fn write_u8(&mut self, ptr: u16, value: u8) {
        self.buffer[ptr as usize] = value;
    }

    pub(crate) fn write_u16(&mut self, ptr: u16, value: u16) {
        let ptr = ptr as usize;
        let values = value.to_be_bytes();
        self.buffer[ptr] = values[0];
        self.buffer[ptr + 1] = values[1];
    }

    pub(crate) fn copy_from_slice(&mut self, ptr: u16, values: &[u8]) {
        self.buffer[(ptr as usize)..(ptr as usize) + values.len()].copy_from_slice(values);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_load_store() {
        let mut memory = Memory::new();
        memory.write_u16(2, 0x200);
        assert_eq!(0x200, memory.read_u16(2));
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
        self.ptr -= 1;
        let value = self.buffer[self.ptr];
        value
    }
}
