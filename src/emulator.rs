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
        let mut memory = Memory::new();
        Self::load_font_sprites(&mut memory);
        Self {
            cpu: Cpu::new(),
            memory,
            stack: Stack::new(),
            display: DisplayBuffer::new(),
        }
    }

    fn load_font_sprites(memory: &mut Memory) {
        memory.copy_from_slice(
            0x050,
            &[
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80, // F
            ],
        );
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
            Command::Load { register, value } => self.load(register, value),
            Command::CopyRegister { write, read } => self.copy_register(write, read),
            Command::LoadI { value } => self.load_i(value),
            Command::Add { register, value } => self.add(register, value),
            Command::AddRegisters { write, read } => self.add_registers(write, read),
            Command::AddI { read } => self.add_i(read),
            Command::JumpOffset { address } => self.jump_offset(address),
            Command::Call { address } => self.call_subroutine(address),
            Command::LoadSpriteDigitIntoI { read_register } => todo!(),
            Command::LoadBcd { read_register } => todo!(),
            Command::Or { write, read } => self.or(write, read),
            Command::And { write, read } => self.and(write, read),
            Command::Xor { write, read } => self.xor(write, read),
            Command::Sub { write, read } => todo!(),
            Command::SubInverse { write, read } => todo!(),
            Command::Shr { write, read } => todo!(),
            Command::Shl { write, read } => todo!(),
            Command::RandomAnd { register, value } => todo!(),
            Command::DrawSprite {
                register_x,
                register_y,
                value,
            } => todo!(),
            Command::SkipIfKeyPressed { key_register } => todo!(),
            Command::SkipIfKeyNotPressed { key_register } => todo!(),
            Command::LoadDelay { register } => todo!(),
            Command::SetDelay { register } => todo!(),
            Command::SetSound { register } => todo!(),
            Command::WaitKeyPress { register, key } => todo!(),
            Command::DumpAll { until_register } => todo!(),
            Command::LoadAll { until_register } => todo!(),
            Command::NoOp => todo!(),
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

    fn call_subroutine(&mut self, address: u16) {
        self.stack.push(*self.cpu.pc());
        *self.cpu.pc_mut() = address;
    }

    fn jump(&mut self, address: u16) {
        *self.cpu.pc_mut() = address;
    }

    fn jump_offset(&mut self, address: u16) {
        self.jump(address + *self.cpu.register(0) as u16);
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
    fn load(&mut self, register: u8, value: u8) {
        *self.cpu.register_mut(register) = value;
    }
    fn copy_register(&mut self, write: u8, read: u8) {
        *self.cpu.register_mut(write) = *self.cpu.register(read);
    }
    fn load_i(&mut self, value: u16) {
        *self.cpu.i_mut() = value;
    }
    fn add(&mut self, register: u8, value: u8) {
        *self.cpu.register_mut(register) += value;
    }
    fn add_registers(&mut self, write: u8, read: u8) {
        let a = self.cpu.register(write);
        let b = self.cpu.register(read);
        if (*a as u16) + (*b as u16) > 255 {
            *self.cpu.register_mut(write) = a.wrapping_add(*b);
            self.cpu.carry_on();
        } else {
            *self.cpu.register_mut(write) = a + b;
            self.cpu.carry_off();
        }
    }
    fn add_i(&mut self, register: u8) {
        *self.cpu.i_mut() += *self.cpu.register(register) as u16;
    }

    fn or(&mut self, write: u8, read: u8) {
        *self.cpu.register_mut(write) |= *self.cpu.register(read);
    }
    fn and(&mut self, write: u8, read: u8) {
        *self.cpu.register_mut(write) &= *self.cpu.register(read);
    }
    fn xor(&mut self, write: u8, read: u8) {
        *self.cpu.register_mut(write) ^= *self.cpu.register(read);
    }
    fn sub(&mut self, write: u8, read: u8) {
        let a = *self.cpu.register(write);
        let b = *self.cpu.register(read);
        *self.cpu.register_mut(write) = self.sub_with_borrow(a, b);
    }
    fn sub_inverse(&mut self, write: u8, read: u8) {
        let a = *self.cpu.register(write);
        let b = *self.cpu.register(read);
        *self.cpu.register_mut(write) = self.sub_with_borrow(b, a);
    }
    fn sub_with_borrow(&mut self, a: u8, b: u8) -> u8 {
        if a < b {
            self.cpu.carry_on();
            a.wrapping_sub(b)
        } else {
            a - b
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
        *emulator.cpu.register_mut(0) = 0x12;

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
        *emulator.cpu.register_mut(1) = 0x12;
        emulator.tick();
        assert_eq!(ptr_start + 12, *emulator.cpu.pc());

        // Values stored in registers 0 and 1 are not equal
        emulator.memory.store(ptr_start + 12, 0x9010);
        *emulator.cpu.register_mut(0) = 0x11;
        emulator.tick();
        assert_eq!(ptr_start + 16, *emulator.cpu.pc());
    }

    #[test]
    fn can_load() {
        let mut emulator = Emulator::new();
        let ptr = CHIP8_START as u16;
        emulator.memory.store(ptr, 0x6012);

        // Load 0x12 into register 0
        assert_ne!(*emulator.cpu.register(0), 0x12);
        emulator.tick();
        assert_eq!(*emulator.cpu.register(0), 0x12);

        // Copy the content of register 0 into register 5
        emulator.memory.store(ptr + 2, 0x8500);
        emulator.tick();
        assert_eq!(*emulator.cpu.register(5), 0x12);

        // Load 0x0300 into register I
        emulator.memory.store(ptr + 4, 0xA300);
        emulator.tick();
        assert_eq!(*emulator.cpu.i(), 0x0300);
    }

    #[test]
    fn can_add() {
        let mut emulator = Emulator::new();
        let ptr = CHIP8_START as u16;
        emulator.memory.store(ptr, 0x7112);
        *emulator.cpu.register_mut(1) = 0x05;

        // Add 0x12 to whatever is stored in register 1
        emulator.tick();
        assert_eq!(0x05 + 0x12, *emulator.cpu.register(1));

        // Store 0x03 in register 2 and add registers 1 and 2
        *emulator.cpu.register_mut(2) = 0x03;
        emulator.cpu.carry_on();
        emulator.memory.store(ptr + 2, 0x8124);
        emulator.tick();
        assert_eq!(0x05 + 0x12 + 0x03, *emulator.cpu.register(1));
        assert_eq!(0, *emulator.cpu.carry());

        // Add whatever is stored in register 1 to register I
        emulator.memory.store(ptr + 4, 0xF11E);
        emulator.tick();
        assert_eq!(0x05 + 0x12 + 0x03, *emulator.cpu.i());
    }
}
