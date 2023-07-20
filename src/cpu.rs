use crate::memory::CHIP8_START;

/// The [`CPU`] Hosts all the registers and gates
/// access to them.
#[derive(Default)]
pub(crate) struct Cpu {
    /// The program counter pointer to the currently
    /// executed instruction in memory
    pc: u16,
    pub(crate) v0: u8,
    pub(crate) v1: u8,
    pub(crate) v2: u8,
    pub(crate) v3: u8,
    pub(crate) v4: u8,
    pub(crate) v5: u8,
    pub(crate) v6: u8,
    pub(crate) v7: u8,
    pub(crate) v8: u8,
    pub(crate) v9: u8,
    pub(crate) va: u8,
    pub(crate) vb: u8,
    pub(crate) vc: u8,
    pub(crate) vd: u8,
    pub(crate) ve: u8,
    pub(crate) vf: u8,
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
}
