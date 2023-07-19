/// The [`CPU`] Hosts all the registers and gates
/// access to them.
pub(crate) struct Cpu {
    /// The program counter pointer to the currently
    /// executed instruction in memory
    pc: u16,
    /// The stack pointer
    sp: u8,
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
}
