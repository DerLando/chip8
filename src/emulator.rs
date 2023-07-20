use crate::{
    cpu::Cpu,
    display::DisplayBuffer,
    memory::{Memory, Stack},
    opcode::OpCode,
};

/// The main emulator
pub struct Emulator {
    pub(crate) cpu: Cpu,
    pub(crate) memory: Memory,
    pub(crate) stack: Stack,
    pub(crate) display: DisplayBuffer,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            stack: Stack::new(),
            display: DisplayBuffer::new(),
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.load_op();
        let opcode = opcode.into();
        self.execute(opcode);
    }

    fn load_op(&mut self) -> u16 {
        let pc = self.cpu.pc_mut();
        let opcode = self.memory.load(*pc);
        *pc += 2;

        opcode
    }

    fn execute(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::ClearScreen(_) => self.clear_screen(),
            OpCode::Return(_) => self.return_from_subroutine(),
            OpCode::Jump(value) => self.jump(value),
            _ => unreachable!(),
        }
    }
}

/// Interpreter
impl Emulator {
    fn clear_screen(&mut self) {
        self.display.clear()
    }
    fn return_from_subroutine(&mut self) {
        *self.cpu.pc_mut() = self.stack.pop();
    }

    fn jump(&mut self, raw_value: u16) {
        let address = raw_value << 4;
        let address = address >> 4;
        *self.cpu.pc_mut() = address;
    }
}

#[cfg(test)]
mod test {
    use crate::memory::CHIP8_START;

    use super::*;

    #[test]
    fn can_jump() {
        let mut emulator = Emulator::new();
        emulator.memory.store(CHIP8_START as u16, 0x1300);

        assert_eq!(CHIP8_START as u16, *emulator.cpu.pc());
        emulator.tick();
        println!("{:4X}", emulator.cpu.pc());
        assert_eq!(0x0300, *emulator.cpu.pc());
    }
}
