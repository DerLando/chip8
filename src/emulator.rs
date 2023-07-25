use crate::{
    command::Command,
    config::{DumpLoadStyle, EmulatorConfiguration, JumpOffsetStyle, ShiftStyle},
    cpu::Cpu,
    display::DisplayBuffer,
    io::{keyboard::Keyboard, timer::Timer},
    memory::{Memory, Stack, CHIP8_START},
    opcode::OpCode,
};

/// The main emulator
pub struct Emulator {
    pub configuration: EmulatorConfiguration,
    pub(crate) cpu: Cpu,
    pub(crate) memory: Memory,
    pub(crate) stack: Stack,
    pub(crate) display: DisplayBuffer,
    pub(crate) keyboard: Keyboard,
    pub(crate) delay_timer: Timer,
    pub(crate) sound_timer: Timer,
    rng: oorandom::Rand32,
}

impl Emulator {
    pub fn new() -> Self {
        let mut memory = Memory::new();
        Self::load_font_sprites(&mut memory);
        Self {
            configuration: EmulatorConfiguration::default(),
            cpu: Cpu::new(),
            memory,
            stack: Stack::new(),
            display: DisplayBuffer::new(),
            keyboard: Keyboard::new(),
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
            rng: oorandom::Rand32::new(42),
        }
    }

    pub fn with_rom(mut self, rom: &[u8]) -> Self {
        self.memory.copy_from_slice(CHIP8_START as u16, rom);
        self
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.cpu = Cpu::new();
        self.memory.clear_public();
        self.stack = Stack::new();
        self.display.clear();
        self.memory.copy_from_slice(CHIP8_START as u16, rom);
    }

    pub fn load_test_rom(&mut self) {
        self.load_rom(include_bytes!("../roms/test_opcode.ch8"))
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

    fn font_sprite_address(character: u8) -> u16 {
        0x050 + character as u16 * 5
    }

    /// Perform a single, atomic tick of the emulator.
    /// This follows the basic cpu loop of:
    /// - Load
    /// - Decode
    /// - Execute
    pub fn tick(&mut self) {
        self.update_delay_register();
        self.update_sound_register();

        // Load
        let opcode = self.load_op();

        // Decode
        let opcode: OpCode = opcode.into();
        let command = opcode.into();

        // Execute
        self.execute(command);
    }

    fn update_delay_register(&mut self) {
        if *self.cpu.delay() > 0 {
            let steps = self.delay_timer.tick();
            if steps > *self.cpu.delay() {
                *self.cpu.delay_mut() = 0;
            } else {
                *self.cpu.delay_mut() -= steps;
            }
        }
    }

    fn update_sound_register(&mut self) {
        if *self.cpu.sound() > 0 {
            let steps = self.sound_timer.tick();
            if steps > *self.cpu.sound() {
                *self.cpu.sound_mut() = 0;
            } else {
                *self.cpu.sound_mut() -= steps;
            }
        }
    }

    fn load_op(&mut self) -> u16 {
        let opcode = self.memory.read_u16(*self.cpu.pc());
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
            Command::JumpOffset { address, register } => match self.configuration.jump {
                JumpOffsetStyle::OffsetFromV0 => self.jump_offset(address),
                JumpOffsetStyle::OffsetVariable => self.jump_offset_variable(address, register),
            },
            Command::Call { address } => self.call_subroutine(address),
            Command::LoadSpriteDigitIntoI { read_register } => {
                self.load_sprite_key_into_i(read_register)
            }
            Command::LoadBcd { read_register } => self.load_bcd(read_register),
            Command::Or { write, read } => self.or(write, read),
            Command::And { write, read } => self.and(write, read),
            Command::Xor { write, read } => self.xor(write, read),
            Command::Sub { write, read } => self.sub(write, read),
            Command::SubInverse { write, read } => self.sub_inverse(write, read),
            Command::ShiftRight { write, read } => match self.configuration.shift {
                ShiftStyle::CopyThenShift => self.shift_right(write, read),
                ShiftStyle::ShiftInPlace => self.shift_right_in_place(write),
            },
            Command::ShiftLeft { write, read } => match self.configuration.shift {
                ShiftStyle::CopyThenShift => self.shift_left(write, read),
                ShiftStyle::ShiftInPlace => self.shift_left_in_place(write),
            },
            Command::RandomAnd { register, value } => self.random_and(register, value),
            Command::DrawSprite {
                register_x,
                register_y,
                value,
            } => self.draw(register_x, register_y, value),
            Command::SkipIfKeyPressed { key_register } => self.skip_if_key_pressed(key_register),
            Command::SkipIfKeyNotPressed { key_register } => {
                self.skip_if_key_not_pressed(key_register)
            }
            Command::LoadDelay { register } => self.load_delay(register),
            Command::SetDelay { register } => self.set_delay(register),
            Command::SetSound { register } => self.set_sound(register),
            Command::WaitKeyPress { register, key } => self.wait_key(register, key),
            Command::DumpAll { until_register } => match self.configuration.r_register {
                DumpLoadStyle::AffectIRegister => self.dump_all_variable(until_register),
                DumpLoadStyle::StaticIRegister => self.dump_all_static(until_register),
            },
            Command::LoadAll { until_register } => match self.configuration.r_register {
                DumpLoadStyle::AffectIRegister => self.load_all_variable(until_register),
                DumpLoadStyle::StaticIRegister => self.load_all_static(until_register),
            },
            Command::NoOp => log::warn!("Invalid instruction!"),
        }
    }
}

/// Peripherals implementations
impl Emulator {
    pub fn press_key(&mut self, key: u8) {
        self.keyboard.press(key);
    }

    pub fn release_key(&mut self, key: u8) {
        self.keyboard.release(key);
    }

    pub fn is_sound_on(&self) -> bool {
        *self.cpu.sound() > 0
    }

    pub fn is_pixel_on(&self, x: u8, y: u8) -> bool {
        self.display.is_pixel_on(x, y)
    }

    pub fn dump_registers(&self) -> [u8; 16] {
        [
            *self.cpu.register(0),
            *self.cpu.register(1),
            *self.cpu.register(2),
            *self.cpu.register(3),
            *self.cpu.register(4),
            *self.cpu.register(5),
            *self.cpu.register(6),
            *self.cpu.register(7),
            *self.cpu.register(8),
            *self.cpu.register(9),
            *self.cpu.register(10),
            *self.cpu.register(11),
            *self.cpu.register(12),
            *self.cpu.register(13),
            *self.cpu.register(14),
            *self.cpu.register(15),
        ]
    }

    pub fn pc(&self) -> u16 {
        *self.cpu.pc()
    }
    pub fn i(&self) -> u16 {
        *self.cpu.i()
    }
    pub fn delay(&self) -> u8 {
        *self.cpu.delay()
    }
    pub fn dump_raw_memory_around_pc(&self) -> [u8; 11] {
        [
            self.memory.read_u8(self.pc() - 5),
            self.memory.read_u8(self.pc() - 4),
            self.memory.read_u8(self.pc() - 3),
            self.memory.read_u8(self.pc() - 2),
            self.memory.read_u8(self.pc() - 1),
            self.memory.read_u8(self.pc() - 0),
            self.memory.read_u8(self.pc() + 1),
            self.memory.read_u8(self.pc() + 2),
            self.memory.read_u8(self.pc() + 3),
            self.memory.read_u8(self.pc() + 4),
            self.memory.read_u8(self.pc() + 5),
        ]
    }
    pub fn dump_double_memory_around_pc(&self) -> [u16; 11] {
        [
            self.memory.read_u16(self.pc() - 10),
            self.memory.read_u16(self.pc() - 8),
            self.memory.read_u16(self.pc() - 6),
            self.memory.read_u16(self.pc() - 4),
            self.memory.read_u16(self.pc() - 2),
            self.memory.read_u16(self.pc() - 0),
            self.memory.read_u16(self.pc() + 2),
            self.memory.read_u16(self.pc() + 4),
            self.memory.read_u16(self.pc() + 6),
            self.memory.read_u16(self.pc() + 8),
            self.memory.read_u16(self.pc() + 10),
        ]
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

    fn jump_offset_variable(&mut self, address: u16, register: u8) {
        self.jump(address + *self.cpu.register(register) as u16);
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

    fn skip_if_key_pressed(&mut self, key_register: u8) {
        if self.keyboard.is_pressed(*self.cpu.register(key_register)) {
            self.cpu.advance_pc();
        }
    }

    fn skip_if_key_not_pressed(&mut self, key_register: u8) {
        if !self.keyboard.is_pressed(*self.cpu.register(key_register)) {
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
    fn load_sprite_key_into_i(&mut self, key_register: u8) {
        *self.cpu.i_mut() = Self::font_sprite_address(*self.cpu.register(key_register));
    }
    fn load_bcd(&mut self, read: u8) {
        let value = *self.cpu.register(read);
        let address = *self.cpu.i();
        self.memory.write_u8(address + 0, value / 100);
        self.memory.write_u8(address + 1, (value / 10) % 10);
        self.memory.write_u8(address + 2, value % 10);
    }
    fn add(&mut self, register: u8, value: u8) {
        *self.cpu.register_mut(register) = self.cpu.register(register).wrapping_add(value);
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
    fn random_and(&mut self, register: u8, value: u8) {
        *self.cpu.register_mut(register) = value & (self.rng.rand_u32() >> 24) as u8;
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
            self.cpu.carry_off();
            a.wrapping_sub(b)
        } else {
            self.cpu.carry_on();
            a - b
        }
    }

    /// Shifting is ambiguous, older versions copied over the value
    /// from the read register to the write register, while newer
    /// versions shift in-place the given register.
    fn shift_right(&mut self, write: u8, read: u8) {
        self.copy_register(write, read);
        self.shift_right_in_place(write);
    }

    fn shift_right_in_place(&mut self, register: u8) {
        let rightmost = *self.cpu.register(register) & 1;
        *self.cpu.register_mut(register) >>= 1;
        if rightmost == 1 {
            self.cpu.carry_on();
        } else {
            self.cpu.carry_off();
        }
    }

    fn shift_left(&mut self, write: u8, read: u8) {
        self.copy_register(write, read);
        self.shift_left_in_place(write);
    }

    fn shift_left_in_place(&mut self, register: u8) {
        let leftmost = *self.cpu.register(register) & 0b1000_0000;
        *self.cpu.register_mut(register) <<= 1;
        if leftmost == 0b1000_0000 {
            self.cpu.carry_on();
        } else {
            self.cpu.carry_off();
        }
    }

    fn load_all_static(&mut self, until_register: u8) {
        let mut start_address = *self.cpu.i();
        for i in 0..=until_register {
            *self.cpu.register_mut(i) = self.memory.read_u8(start_address + i as u16);
        }
    }

    fn load_all_variable(&mut self, until_register: u8) {
        for i in 0..=until_register {
            *self.cpu.i_mut() += i as u16;
            *self.cpu.register_mut(i) = self.memory.read_u8(*self.cpu.i());
        }
    }

    fn dump_all_static(&mut self, until_register: u8) {
        let mut start_address = *self.cpu.i();
        for i in 0..=until_register {
            self.memory
                .write_u8(start_address + i as u16, *self.cpu.register(i));
        }
    }

    fn dump_all_variable(&mut self, until_register: u8) {
        for i in 0..=until_register {
            *self.cpu.i_mut() += i as u16;
            self.memory.write_u8(*self.cpu.i(), *self.cpu.register(i));
        }
    }

    fn draw(&mut self, register_x: u8, register_y: u8, value: u8) {
        let x = *self.cpu.register(register_x) % 64;
        let y = *self.cpu.register(register_y) % 32;
        let height = value;
        let start_address = *self.cpu.i();
        let mut did_turn_off_pixel = false;

        for (y_offset, address) in (start_address..start_address + height as u16).enumerate() {
            let y_pos = y as usize + y_offset;
            if y_pos > 32 {
                break;
            }
            let y_pos = y_pos as u8;

            // Bits are right-to-left, but we draw left-to right
            // so we need to reverse the sprite bits after reading
            let sprite_row = self.memory.read_u8(address).reverse_bits();
            for x_offset in 0..u8::BITS {
                let x_pos = x as u32 + x_offset;
                if x_pos > 64 {
                    break;
                }
                let x_pos = x_pos as u8;

                let should_flip = sprite_row >> x_offset & 1 == 1;
                if !should_flip {
                    continue;
                }

                did_turn_off_pixel |= self.display.flip_pixel(x_pos, y_pos);
            }
        }

        if did_turn_off_pixel {
            self.cpu.carry_on();
        }
    }

    fn wait_key(&mut self, key_register: u8, key: u8) {
        if self.keyboard.is_pressed(key) {
            *self.cpu.register_mut(key_register) = key;
        } else {
            self.cpu.rollback_pc();
        }
    }

    fn load_delay(&mut self, register: u8) {
        *self.cpu.register_mut(register) = *self.cpu.delay();
    }

    fn set_delay(&mut self, register: u8) {
        self.delay_timer.tick();
        *self.cpu.delay_mut() = *self.cpu.register(register);
    }

    fn set_sound(&mut self, register: u8) {
        self.sound_timer.tick();
        *self.cpu.sound_mut() = *self.cpu.register(register);
    }
}

#[cfg(test)]
mod test {
    use crate::memory::CHIP8_START;

    use super::*;

    #[test]
    fn can_jump() {
        let mut emulator = Emulator::new();
        emulator.memory.write_u16(CHIP8_START as u16, 0x1300);

        assert_eq!(CHIP8_START as u16, *emulator.cpu.pc());
        emulator.tick();
        assert_eq!(0x0300, *emulator.cpu.pc());
    }

    #[test]
    fn can_skip_instructions() {
        let mut emulator = Emulator::new();
        let ptr_start = CHIP8_START as u16;
        emulator.memory.write_u16(ptr_start, 0x3012);
        *emulator.cpu.register_mut(0) = 0x12;

        // Value equals value stored in register 0
        assert_eq!(ptr_start, *emulator.cpu.pc());
        emulator.tick();
        assert_eq!(ptr_start + 4, *emulator.cpu.pc());

        // Value not equals value stored in register 0
        emulator.memory.write_u16(ptr_start + 4, 0x4005);
        emulator.tick();
        assert_eq!(ptr_start + 8, *emulator.cpu.pc());

        // Values stored in registers 0 and 1 are equal
        emulator.memory.write_u16(ptr_start + 8, 0x5010);
        *emulator.cpu.register_mut(1) = 0x12;
        emulator.tick();
        assert_eq!(ptr_start + 12, *emulator.cpu.pc());

        // Values stored in registers 0 and 1 are not equal
        emulator.memory.write_u16(ptr_start + 12, 0x9010);
        *emulator.cpu.register_mut(0) = 0x11;
        emulator.tick();
        assert_eq!(ptr_start + 16, *emulator.cpu.pc());
    }

    #[test]
    fn can_load() {
        let mut emulator = Emulator::new();
        let ptr = CHIP8_START as u16;
        emulator.memory.write_u16(ptr, 0x6012);

        // Load 0x12 into register 0
        assert_ne!(*emulator.cpu.register(0), 0x12);
        emulator.tick();
        assert_eq!(*emulator.cpu.register(0), 0x12);

        // Copy the content of register 0 into register 5
        emulator.memory.write_u16(ptr + 2, 0x8500);
        emulator.tick();
        assert_eq!(*emulator.cpu.register(5), 0x12);

        // Load 0x0300 into register I
        emulator.memory.write_u16(ptr + 4, 0xA300);
        emulator.tick();
        assert_eq!(*emulator.cpu.i(), 0x0300);
    }

    #[test]
    fn can_add() {
        let mut emulator = Emulator::new();
        let ptr = CHIP8_START as u16;
        emulator.memory.write_u16(ptr, 0x7112);
        *emulator.cpu.register_mut(1) = 0x05;

        // Add 0x12 to whatever is stored in register 1
        emulator.tick();
        assert_eq!(0x05 + 0x12, *emulator.cpu.register(1));

        // Store 0x03 in register 2 and add registers 1 and 2
        *emulator.cpu.register_mut(2) = 0x03;
        emulator.cpu.carry_on();
        emulator.memory.write_u16(ptr + 2, 0x8124);
        emulator.tick();
        assert_eq!(0x05 + 0x12 + 0x03, *emulator.cpu.register(1));
        assert_eq!(0, *emulator.cpu.carry());

        // Add whatever is stored in register 1 to register I
        emulator.memory.write_u16(ptr + 4, 0xF11E);
        emulator.tick();
        assert_eq!(0x05 + 0x12 + 0x03, *emulator.cpu.i());
    }

    #[test]
    fn can_bcd() {
        let mut emulator = Emulator::new();
        emulator.memory.write_u16(CHIP8_START as u16, 0xF033);
        *emulator.cpu.register_mut(0) = 234;
        *emulator.cpu.i_mut() = 0x0300;

        emulator.tick();
        assert_eq!(2, emulator.memory.read_u8(*emulator.cpu.i() + 0));
        assert_eq!(3, emulator.memory.read_u8(*emulator.cpu.i() + 1));
        assert_eq!(4, emulator.memory.read_u8(*emulator.cpu.i() + 2));
    }

    #[test]
    #[cfg(feature = "std")]
    fn can_run_timers() {
        let mut emulator = Emulator::new();
        *emulator.cpu.register_mut(0) = 60;
        emulator.memory.write_u16(CHIP8_START as u16, 0xF015);

        emulator.tick();
        assert_eq!(60, *emulator.cpu.delay());

        std::thread::sleep(core::time::Duration::from_millis(500));
        emulator.tick();
        assert_eq!(30, *emulator.cpu.delay());
    }

    #[test]
    fn can_run_subroutines() {
        let mut emulator = Emulator::new();
        let subroutine_address = 0x0300;
        emulator.memory.write_u16(CHIP8_START as u16, 0x2300);
        emulator.memory.write_u16(subroutine_address, 0x00EE);

        assert_eq!(CHIP8_START as u16, *emulator.cpu.pc());
        emulator.tick();
        assert_eq!(subroutine_address, *emulator.cpu.pc());
        emulator.tick();
        assert_eq!(CHIP8_START as u16 + 2, *emulator.cpu.pc());
    }

    #[test]
    #[cfg(feature = "std")]
    fn passes_bc_test_rom() {
        let rom = include_bytes!("../roms/BC_test.ch8");
        let mut emulator = Emulator::new().with_rom(rom);
        // emulator.configuration.shift = ShiftStyle::CopyThenShift;

        for _ in 0..400 {
            emulator.tick();
        }

        println!("{}", emulator.display);
        assert_eq!(
            "◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◻◻◻◻◻◼◼◼◼◻◻◻◼◻◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◻◻◻◼◻◻◻◼◻◻◻◻◼◻◻◼◼◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◻◻◻◼◻◻◻◼◻◻◻◻◼◻◻◼◻◼◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◻◻◻◻◼◻◻◻◻◼◻◻◼◻◻◼◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◻◻◻◼◻◻◻◼◻◻◻◻◼◻◻◼◻◻◻◼◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◻◻◻◼◻◻◻◼◻◻◻◻◼◻◻◼◻◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◻◻◻◼◻◻◻◼◻◻◻◻◼◻◻◼◻◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◻◻◻◻◻◼◼◼◼◻◻◻◼◻◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◼◼◻◻◻◻◻◻◻◻◻◻◻◻◻◼◼◻◻◻◻◻◻◻◻◻◻◻◻◻◼◻◻◻◻◼◼◼◻◻◻◻◻◻◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◼◻◼◻◻◻◻◻◻◻◻◻◻◻◻◼◻◼◻◻◻◻◻◻◻◻◻◻◻◻◼◻◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◼◻◼◻◻◼◻◼◻◻◻◻◻◻◻◼◻◼◻◻◻◼◼◻◻◻◼◼◻◻◼◼◻◻◻◼◻◻◻◻◻◼◻◻◻◻◻◼◻◻◻◼◼◻◻◻◻◻◻◻◻◻
◻◻◼◼◻◻◻◼◻◼◻◻◻◻◻◻◻◼◼◻◻◻◼◻◼◻◻◼◻◻◻◻◼◻◻◻◻◼◻◻◻◻◼◻◼◻◻◻◼◼◻◻◼◻◼◻◻◻◼◼◻◻◻◻
◻◻◼◻◼◻◻◼◼◼◻◻◻◻◻◻◻◼◻◼◻◻◼◼◻◻◻◻◼◻◻◻◼◻◻◻◻◼◻◻◻◻◼◻◼◻◻◼◻◼◻◻◼◼◻◻◻◻◼◻◻◻◻◻
◻◻◼◻◼◻◻◻◻◼◻◻◻◻◻◻◻◼◻◼◻◻◼◻◻◻◻◻◻◼◻◻◼◻◻◻◻◼◻◻◻◻◼◻◼◻◻◼◻◼◻◻◼◻◻◻◻◻◼◻◻◻◻◻
◻◻◼◼◻◻◻◻◻◼◻◻◻◻◻◻◻◼◼◻◻◻◻◼◼◻◻◼◼◻◻◻◻◼◼◻◻◼◼◼◻◻◻◼◻◻◻◻◼◼◻◻◻◼◼◻◻◻◼◻◼◻◻◻
◻◻◻◻◻◻◻◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
",
            format!("{}", emulator.display),
        )
    }

    #[cfg(feature = "std")]
    #[test]
    // #[ignore]
    fn passes_opcode_test_rom() {
        let rom = include_bytes!("../roms/test_opcode.ch8");
        let mut emulator = Emulator::new().with_rom(rom);

        for _ in 0..400 {
            emulator.tick();
        }

        println!("{}", emulator.display);
        assert_eq!(
            "◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◼◼◼◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◼◼◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◼◼◻◻◼◼◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◻◼◼◻◻◼◻◻◻◼◻◼◻◼◼◻◻◻◻◻◻◻◼◻◼◻◼◼◻◻◻◼◻◼◻◼◼◻◻◻◻◻◻◼◼◼◻◻◼◻◻◼◻◼◻◼◼◻◻◻◻◻◻
◻◻◻◼◻◼◻◼◻◻◼◻◼◻◼◻◼◻◻◻◻◻◻◼◻◼◻◼◻◻◻◻◼◻◼◻◼◻◼◻◻◻◻◻◼◻◼◻◻◻◼◻◼◻◼◻◼◻◼◻◻◻◻◻
◻◼◼◼◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◼◼◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◼◼◻◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◼◻◼◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◼◼◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◼◼◻◼◼◼◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◼◼◼◻◻◼◻◻◻◼◻◼◻◼◼◻◻◻◻◻◻◻◼◼◼◻◼◻◼◻◻◼◻◼◻◼◼◻◻◻◻◻◻◼◼◼◻◼◻◻◻◼◻◼◻◼◼◻◻◻◻◻◻
◻◻◻◼◻◼◻◼◻◻◼◻◼◻◼◻◼◻◻◻◻◻◻◼◻◼◻◼◻◼◻◻◼◻◼◻◼◻◼◻◻◻◻◻◼◻◼◻◼◼◼◻◼◻◼◻◼◻◼◻◻◻◻◻
◻◻◻◼◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◼◼◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◼◼◻◼◼◼◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◼◼◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◼◼◻◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◼◼◻◼◼◼◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◻◼◻◻◻◼◻◻◻◼◻◼◻◼◼◻◻◻◻◻◻◻◼◼◼◻◻◼◻◻◻◼◻◼◻◼◼◻◻◻◻◻◻◼◼◼◻◼◼◻◻◼◻◼◻◼◼◻◻◻◻◻◻
◻◻◻◼◻◼◻◼◻◻◼◻◼◻◼◻◼◻◻◻◻◻◻◼◻◼◻◻◼◻◻◻◼◻◼◻◼◻◼◻◻◻◻◻◼◻◼◻◼◻◻◻◼◻◼◻◼◻◼◻◻◻◻◻
◻◻◼◻◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◼◼◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◼◼◻◼◼◼◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◼◼◼◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◼◼◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◼◼◻◻◼◼◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◻◻◼◻◻◼◻◻◻◼◻◼◻◼◼◻◻◻◻◻◻◻◼◼◼◻◻◻◼◻◻◼◻◼◻◼◼◻◻◻◻◻◻◼◻◻◻◻◼◻◻◼◻◼◻◼◼◻◻◻◻◻◻
◻◻◻◼◻◼◻◼◻◻◼◻◼◻◼◻◼◻◻◻◻◻◻◼◻◼◻◼◼◻◻◻◼◻◼◻◼◻◼◻◻◻◻◻◼◼◻◻◻◻◼◻◼◻◼◻◼◻◼◻◻◻◻◻
◻◻◻◼◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◼◼◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◻◻◻◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◼◼◼◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◼◼◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◼◼◻◼◼◼◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◼◼◼◻◻◼◻◻◻◼◻◼◻◼◼◻◻◻◻◻◻◻◼◼◼◻◻◼◼◻◻◼◻◼◻◼◼◻◻◻◻◻◻◼◻◻◻◻◼◼◻◼◻◼◻◼◼◻◻◻◻◻◻
◻◻◻◼◻◼◻◼◻◻◼◻◼◻◼◻◼◻◻◻◻◻◻◼◻◼◻◻◻◼◻◻◼◻◼◻◼◻◼◻◻◻◻◻◼◼◻◻◻◻◼◻◼◻◼◻◼◻◼◻◻◻◻◻
◻◼◼◼◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◼◼◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◻◻◻◼◼◼◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◼◻◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◼◻◻◼◻◼◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◼◻◼◻◻◼◻◻◻◼◻◼◻◼◼◻◻◻◻◻◻◻◼◼◼◻◼◼◼◻◻◼◻◼◻◼◼◻◻◻◻◻◻◻◼◻◻◻◼◻◻◼◻◼◻◼◼◻◻◻◻◻◻
◻◼◼◼◻◼◻◼◻◻◼◻◼◻◼◻◼◻◻◻◻◻◻◼◻◼◻◻◻◼◻◻◼◻◼◻◼◻◼◻◻◻◻◻◻◼◻◻◼◻◼◻◼◻◼◻◼◻◼◻◻◻◻◻
◻◼◻◼◻◼◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◻◼◼◼◻◻◻◼◻◻◼◼◼◻◼◻◼◻◻◻◻◻◼◼◼◻◼◻◼◻◼◼◼◻◼◻◼◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
",
            format!("{}", emulator.display)
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn can_draw_ibm_logo() {
        let rom = include_bytes!("../roms/IBM_Logo.ch8");
        let mut emulator = Emulator::new().with_rom(rom);

        for _ in 0..21 {
            emulator.tick();
        }

        println!("{}", emulator.display);
        assert_eq!(
            "◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◻◼◼◼◼◼◼◼◼◼◻◻◻◼◼◼◼◼◻◻◻◻◻◻◻◻◻◼◼◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◻◼◼◼◼◼◼◼◼◼◼◼◻◼◼◼◼◼◼◻◻◻◻◻◻◻◼◼◼◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◻◻◻◻◻◼◼◼◻◻◻◼◼◼◻◻◻◼◼◼◼◼◻◻◻◻◻◼◼◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◻◻◻◻◻◼◼◼◼◼◼◼◻◻◻◻◻◼◼◼◼◼◼◼◻◼◼◼◼◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◻◻◻◻◻◼◼◼◼◼◼◼◻◻◻◻◻◼◼◼◻◼◼◼◼◼◼◼◻◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◻◻◻◻◻◼◼◼◻◻◻◼◼◼◻◻◻◼◼◼◻◻◼◼◼◼◼◻◻◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◻◼◼◼◼◼◼◼◼◼◼◼◻◼◼◼◼◼◻◻◻◼◼◼◻◻◻◼◼◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◼◼◼◼◼◼◼◼◻◼◼◼◼◼◼◼◼◼◻◻◻◼◼◼◼◼◻◻◻◻◼◻◻◻◻◼◼◼◼◼◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻◻
",
            format!("{}", emulator.display)
        );
    }
}
