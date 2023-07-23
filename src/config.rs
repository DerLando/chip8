pub enum ShiftStyle {
    /// Shift the value in the given register in-place
    ShiftInPlace,
    /// Copy the value from register y to register x, then shift
    /// The value that got copied into the x register
    CopyThenShift,
}
pub enum JumpOffsetStyle {
    /// Always calculate the offset from the value stored in register v0
    OffsetFromV0,
    /// Load the offset dynamically from the register given in the opcode
    OffsetVariable,
}
pub enum DumpLoadStyle {
    /// The original interpreter increments the I register while
    /// performing a register dump / load
    AffectIRegister,
    /// More modern interpreters use a temporary variable while
    /// performing a register dump / load, so the I register stays static
    StaticIRegister,
}

/// The behavior of the emulator can be configured towards the different
/// sometimes conflicting specifications of chip-8 emulation.
/// The default version leans more towards more modern emulation,
/// so if you want to properly playback old roms, you might need
/// to configure the emulator accordingly.
pub struct EmulatorConfiguration {
    pub shift: ShiftStyle,
    pub jump: JumpOffsetStyle,
    pub r_register: DumpLoadStyle,
}

impl Default for EmulatorConfiguration {
    fn default() -> Self {
        Self {
            shift: ShiftStyle::ShiftInPlace,
            jump: JumpOffsetStyle::OffsetVariable,
            r_register: DumpLoadStyle::StaticIRegister,
        }
    }
}
