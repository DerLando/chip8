use crate::memory::CHIP8_START;

/// The [`CPU`] Hosts all the registers and gates
/// access to them.
#[derive(Default)]
pub(crate) struct Cpu {
    /// The program counter pointer to the currently
    /// executed instruction in memory
    pc: u16,
    registers: [u8; 16],
    i: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: CHIP8_START as u16,
            ..Default::default()
        }
    }

    pub(crate) fn pc(&self) -> &u16 {
        &self.pc
    }

    pub(crate) fn pc_mut(&mut self) -> &mut u16 {
        &mut self.pc
    }

    pub(crate) fn advance_pc(&mut self) {
        self.pc += 2;
    }

    pub(crate) fn rollback_pc(&mut self) {
        self.pc -= 2;
    }

    pub(crate) fn register(&self, index: u8) -> &u8 {
        &self.registers[index as usize]
    }
    pub(crate) fn register_mut(&mut self, index: u8) -> &mut u8 {
        &mut self.registers[index as usize]
    }
    pub(crate) fn i(&self) -> &u16 {
        &self.i
    }
    pub(crate) fn i_mut(&mut self) -> &mut u16 {
        &mut self.i
    }
    pub(crate) fn carry(&self) -> &u8 {
        &self.registers[15]
    }
    pub(crate) fn carry_on(&mut self) {
        self.registers[15] = 1;
    }
    pub(crate) fn carry_off(&mut self) {
        self.registers[15] = 0;
    }
}
