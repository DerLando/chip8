use crate::opcode::OpCode;

#[rustfmt::skip]
pub(crate) enum Command {
    ClearScreen,
    ReturnFromSubroutine,
    Jump { address: u16 },
    JumpOffset { address: u16, register: u8 },
    Call { address: u16 },
    SkipIfValueEqual { register: u8, value: u8 },
    SkipIfValueNotEqual { register: u8, value: u8 },
    SkipIfRegisterEqual { register_a: u8, register_b: u8 },
    SkipIfRegisterNotEqual { register_a: u8, register_b: u8 },
    Load { register: u8, value: u8 },
    LoadI { value: u16 },
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
    ShiftRight { write: u8, read: u8 },
    ShiftLeft { write: u8, read: u8 },
    RandomAnd { register: u8, value: u8 },
    DrawSprite { register_x: u8, register_y: u8, value: u8 },
    SkipIfKeyPressed { key_register: u8 },
    SkipIfKeyNotPressed { key_register: u8 },
    LoadDelay {register: u8},
    SetDelay {register: u8},
    SetSound {register: u8},
    WaitKeyPress {register: u8 },
    DumpAll { until_register: u8 },
    LoadAll { until_register: u8 },
    NoOp,
}

impl From<OpCode> for Command {
    fn from(value: OpCode) -> Self {
        match value {
            OpCode::ClearScreen(_) => Command::ClearScreen,
            OpCode::Return(_) => Command::ReturnFromSubroutine,
            OpCode::Jump(value) => Command::Jump {
                address: value.skip_first_nibble(),
            },
            OpCode::JumpV0(value) => Command::JumpOffset {
                address: value.skip_first_nibble(),
                register: value.nibble_1(),
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
            OpCode::SkipIfRegistersAreEqual(value) => Command::SkipIfRegisterEqual {
                register_a: value.nibble_1(),
                register_b: value.nibble_2(),
            },
            OpCode::SkipIfRegistersAreNotEqual(value) => Command::SkipIfRegisterNotEqual {
                register_a: value.nibble_1(),
                register_b: value.nibble_2(),
            },
            OpCode::Load(value) => Command::Load {
                register: value.nibble_1(),
                value: value.back(),
            },
            OpCode::LoadRegister(value) => Command::CopyRegister {
                write: value.nibble_1(),
                read: value.nibble_2(),
            },
            OpCode::LoadI(value) => Command::LoadI {
                value: value.skip_first_nibble(),
            },
            OpCode::Add(value) => Command::Add {
                register: value.nibble_1(),
                value: value.back(),
            },
            OpCode::AddWithCarry(value) => Command::AddRegisters {
                write: value.nibble_1(),
                read: value.nibble_2(),
            },
            OpCode::AddI(value) => Command::AddI {
                read: value.nibble_1(),
            },
            OpCode::Or(value) => Command::Or {
                write: value.nibble_1(),
                read: value.nibble_2(),
            },
            OpCode::And(value) => Command::And {
                write: value.nibble_1(),
                read: value.nibble_2(),
            },
            OpCode::RandomAnd(value) => Command::RandomAnd {
                register: value.nibble_1(),
                value: value.back(),
            },
            OpCode::Xor(value) => Command::Xor {
                write: value.nibble_1(),
                read: value.nibble_2(),
            },
            OpCode::Sub(value) => Command::Sub {
                write: value.nibble_1(),
                read: value.nibble_2(),
            },
            OpCode::SubInverse(value) => Command::SubInverse {
                write: value.nibble_1(),
                read: value.nibble_2(),
            },
            OpCode::Shr(value) => Command::ShiftRight {
                write: value.nibble_1(),
                read: value.nibble_2(),
            },
            OpCode::Shl(value) => Command::ShiftLeft {
                write: value.nibble_1(),
                read: value.nibble_2(),
            },
            OpCode::DrawSprite(value) => Command::DrawSprite {
                register_x: value.nibble_1(),
                register_y: value.nibble_2(),
                value: value.nibble_3(),
            },
            OpCode::SkipIfKeyPressed(value) => Command::SkipIfKeyPressed {
                key_register: value.nibble_1(),
            },
            OpCode::SkipIfKeyNotPressed(value) => Command::SkipIfKeyNotPressed {
                key_register: value.nibble_1(),
            },
            OpCode::WaitKeyPress(value) => Command::WaitKeyPress {
                register: value.nibble_1(),
            },
            OpCode::LoadDelay(value) => Command::LoadDelay {
                register: value.nibble_1(),
            },
            OpCode::SetDelay(value) => Command::SetDelay {
                register: value.nibble_1(),
            },
            OpCode::SetSound(value) => Command::SetSound {
                register: value.nibble_1(),
            },
            OpCode::LoadSprite(value) => Command::LoadSpriteDigitIntoI {
                read_register: value.nibble_1(),
            },
            OpCode::LoadBcd(value) => Command::LoadBcd {
                read_register: value.nibble_1(),
            },
            OpCode::LoadAll(value) => Command::LoadAll {
                until_register: value.nibble_1(),
            },
            OpCode::DumpAll(value) => Command::DumpAll {
                until_register: value.nibble_1(),
            },
            OpCode::Invalid(_) => Command::NoOp,
        }
    }
}

trait OpCodeShift {
    type Output;
    type HalfOutput;
    fn skip_first_nibble(&self) -> Self::Output;
    fn nibble_1(&self) -> Self::HalfOutput;
    fn nibble_2(&self) -> Self::HalfOutput;
    fn nibble_3(&self) -> Self::HalfOutput;
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

    fn nibble_2(&self) -> Self::HalfOutput {
        let result = *self << 8;
        (result >> 12) as u8
    }
    fn nibble_3(&self) -> Self::HalfOutput {
        let result = *self << 12;
        (result >> 12) as u8
    }
}
