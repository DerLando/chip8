use crate::{
    command::Command,
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
        let opcode: OpCode = opcode.into();
        let command = opcode.into();
        self.execute(command);
    }

    fn load_op(&mut self) -> u16 {
        let opcode = self.memory.load(*self.cpu.pc());
        self.cpu.advance_pc();
        opcode
    }

    fn execute(&mut self, command: Command) {
        match command {
            Command::ClearScreen => self.clear_screen(),
            Command::ReturnFromSubroutine => self.return_from_subroutine(),
            Command::Jump { address } => self.jump(address),
            Command::SkipIfValueEqual { register, value } => self.skip_if_value_eq(register, value),
            Command::SkipIfValueNotEqual { register, value } => {
                self.skip_if_value_neq(register, value)
            }

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

    fn jump(&mut self, address: u16) {
        *self.cpu.pc_mut() = address;
    }

    fn skip_if_value_eq(&mut self, register: u8, value: u8) {
        if *self.cpu.register(register) == value {
            self.cpu.advance_pc();
        }
    }
    fn skip_if_value_neq(&mut self, register: u8, value: u8) {
        if *self.cpu.register(register) != value {
            self.cpu.advance_pc();
        }
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

    #[test]
    fn can_skip_instructions() {
        let mut emulator = Emulator::new();
        let ptr_start = CHIP8_START as u16;
        emulator.memory.store(ptr_start, 0x3012);
        emulator.cpu.v0 = 0x12;

        assert_eq!(ptr_start, *emulator.cpu.pc());
        emulator.tick();
        assert_eq!(ptr_start + 4, *emulator.cpu.pc());

        emulator.memory.store(ptr_start + 4, 0x4005);
        emulator.tick();
        assert_eq!(ptr_start + 8, *emulator.cpu.pc());
    }
}
