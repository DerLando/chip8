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

    /// Perform a single, atomic tick of the emulator.
    /// This follows the basic cpu loop of:
    /// - Load
    /// - Decode
    /// - Execute
    pub fn tick(&mut self) {
        // Load
        let opcode = self.load_op();

        // Decode
        let opcode: OpCode = opcode.into();
        let command = opcode.into();

        // Execute
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
            Command::SkipIfRegisterEqual {
                register_a,
                register_b,
            } => self.skip_if_registers_eq(register_a, register_b),
            Command::SkipIfRegisterNotEqual {
                register_a,
                register_b,
            } => self.skip_if_registers_neq(register_a, register_b),

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
    fn skip_if_registers_eq(&mut self, register_a: u8, register_b: u8) {
        if *self.cpu.register(register_a) == *self.cpu.register(register_b) {
            self.cpu.advance_pc();
        }
    }
    fn skip_if_registers_neq(&mut self, register_a: u8, register_b: u8) {
        if *self.cpu.register(register_a) != *self.cpu.register(register_b) {
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

        // Value equals value stored in register 0
        assert_eq!(ptr_start, *emulator.cpu.pc());
        emulator.tick();
        assert_eq!(ptr_start + 4, *emulator.cpu.pc());

        // Value not equals value stored in register 0
        emulator.memory.store(ptr_start + 4, 0x4005);
        emulator.tick();
        assert_eq!(ptr_start + 8, *emulator.cpu.pc());

        // Values stored in registers 0 and 1 are equal
        emulator.memory.store(ptr_start + 8, 0x5010);
        emulator.cpu.v1 = 0x12;
        emulator.tick();
        assert_eq!(ptr_start + 12, *emulator.cpu.pc());

        // Values stored in registers 0 and 1 are not equal
        emulator.memory.store(ptr_start + 12, 0x9010);
        emulator.cpu.v1 = 0x11;
        emulator.tick();
        assert_eq!(ptr_start + 16, *emulator.cpu.pc());
    }
}
