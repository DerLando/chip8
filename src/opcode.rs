use std::borrow::Borrow;

/// All known OpCodes of the Chip8,
/// as well as one variant for invalid opcodes
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum OpCode {
    ClearScreen(u16),
    Return(u16),
    Jump(u16),
    Call(u16),
    SkipIfRegisterEqualsValue(u16),
    SkipIfRegisterNotEqualsValue(u16),
    SkipIfRegistersAreEqual(u16),
    Load(u16),
    Add(u16),
    LoadRegister(u16),
    Or(u16),
    And(u16),
    Xor(u16),
    AddWithCarry(u16),
    Sub(u16),
    Shr(u16), // What is this?
    SubInverse(u16),
    Shl(u16), // What is this?
    SkipIfRegistersAreNotEqual(u16),
    LoadI(u16),
    JumpV0(u16),
    RandomAnd(u16),
    DrawSprite(u16),
    SkipIfKeyPressed(u16),
    SkipIfKeyNotPressed(u16),
    LoadDealay(u16),
    WaitKeyPress(u16),
    SetDelay(u16),
    SetSound(u16),
    AddI(u16),
    LoadSprite(u16),
    LoadBcd(u16),
    DumpAll(u16),
    LoadAll(u16),
    Invalid,
}

impl From<u16> for OpCode {
    fn from(value: u16) -> Self {
        let repr: [char; 4] = format!("{:4X}", value)
            .chars()
            .into_iter()
            .collect::<Vec<_>>()
            .try_into()
            .expect("Valid hex wrapper");
        println!("{:?}", repr);
        match repr {
            [' ', ' ', 'E', _] => match repr[3] {
                '0' => OpCode::ClearScreen(value),
                'E' => OpCode::Return(value),
                _ => OpCode::Invalid,
            },
            ['1', ..] => OpCode::Jump(value),
            _ => OpCode::Invalid,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cls_should_parse() {
        let opcode: u16 = 0x00E0;
        assert_eq!(OpCode::ClearScreen(opcode), opcode.into());
    }
    #[test]
    fn ret_should_parse() {
        let opcode: u16 = 0x00EE;
        assert_eq!(OpCode::Return(opcode), opcode.into());
    }
}
