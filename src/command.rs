use crate::opcode::OpCode;

#[rustfmt::skip]
pub(crate) enum Command {
    ClearScreen,
    ReturnFromSubroutine,
    Jump { address: u16 },
    JumpOffset { address: u16 },
    Call { address: u16 },
    SkipIfValueEqual { register: u8, value: u8 },
    SkipIfValueNotEqual { register: u8, value: u8 },
    SkipIfRegisterEqual { register_a: u8, register_b: u8 },
    SkipIfRegisterNotEqual { register_a: u8, register_b: u8 },
    Load { register: u8, value: u8 },
    LoadI { register: u8, value: u8 },
    LoadSpriteDigitIntoI { read_register: u8 },
    LoadBcd { read_register: u8 },
    Add { register: u8, value: u8 },
    AddRegisters { write: u8, read: u8 },
    AddI { read: u8 },
    CopyRegister { write: u8, read: u8 },
    Or { write: u8, read: u8 },
    And { write: u8, read: u8 },
    Xor { write: u8, read: u8 },
    Sub { write: u8, read: u8 },
    SubInverse { write: u8, read: u8 },
    Shr { write: u8, read: u8 },
    Shl { write: u8, read: u8 },
    RandomAnd { register: u8, value: u8 },
    DrawSprite { register_x: u8, register_y: u8, value: u8 },
    SkipIfKeyPressed { key_register: u8 },
    SkipIfKeyNotPressed { key_register: u8 },
    LoadDelay {register: u8},
    SetDelay {register: u8},
    SetSound {register: u8},
    WaitKeyPress {register: u8, key: u8 },
    DumpAll { until_register: u8 },
    LoadAll { until_register: u8 },
}

impl From<OpCode> for Command {
    fn from(value: OpCode) -> Self {
        match value {
            OpCode::ClearScreen(_) => Command::ClearScreen,
            OpCode::Return(_) => Command::ReturnFromSubroutine,
            OpCode::Jump(value) => Command::Jump {
                address: value.skip_first_nibble(),
            },
            OpCode::Call(value) => Command::Call {
                address: value.skip_first_nibble(),
            },
            OpCode::SkipIfRegisterEqualsValue(value) => Command::SkipIfValueEqual {
                register: value.nibble_1(),
                value: value.back(),
            },
            OpCode::SkipIfRegisterNotEqualsValue(value) => Command::SkipIfValueNotEqual {
                register: value.nibble_1(),
                value: value.back(),
            },

            _ => unreachable!(),
        }
    }
}

trait OpCodeShift {
    type Output;
    type HalfOutput;
    fn skip_first_nibble(&self) -> Self::Output;
    fn nibble_1(&self) -> Self::HalfOutput;
    fn front(&self) -> Self::HalfOutput;
    fn back(&self) -> Self::HalfOutput;
}

impl OpCodeShift for u16 {
    type Output = u16;
    type HalfOutput = u8;

    fn skip_first_nibble(&self) -> Self::Output {
        let result = *self << 4;
        result >> 4
    }

    fn nibble_1(&self) -> Self::HalfOutput {
        let result = *self << 4;
        (result >> 12) as u8
    }

    fn front(&self) -> Self::HalfOutput {
        (*self >> 8) as u8
    }

    fn back(&self) -> Self::HalfOutput {
        let result = *self << 8;
        (result >> 8) as u8
    }
}

