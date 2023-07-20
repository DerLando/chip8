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
        let repr: [char; 4] = raw_opcode_chars(value);
        println!("{:?}", repr);
        match repr {
            [' ', ' ', 'E', _] => match repr[3] {
                '0' => OpCode::ClearScreen(value),
                'E' => OpCode::Return(value),
                _ => OpCode::Invalid,
            },
            ['1', ..] => OpCode::Jump(value),
            ['2', ..] => OpCode::Call(value),
            ['3', ..] => OpCode::SkipIfRegisterEqualsValue(value),
            ['4', ..] => OpCode::SkipIfRegisterNotEqualsValue(value),
            ['5', ..] => OpCode::SkipIfRegistersAreEqual(value),
            ['6', ..] => OpCode::Load(value),
            ['7', ..] => OpCode::Add(value),
            ['8', ..] => decode_8_opcodes(repr, value),
            ['9', ..] => OpCode::SkipIfRegistersAreNotEqual(value),
            _ => OpCode::Invalid,
        }
    }
}

pub(crate) fn raw_opcode_chars(opcode: u16) -> [char; 4] {
    format!("{:4X}", opcode)
        .chars()
        .into_iter()
        .collect::<Vec<_>>()
        .try_into()
        .expect("Valid hex wrapper")
}

fn decode_8_opcodes(repr: [char; 4], value: u16) -> OpCode {
    match repr {
        ['8', _, _, '0'] => OpCode::LoadRegister(value),
        ['8', _, _, '1'] => OpCode::Or(value),
        ['8', _, _, '2'] => OpCode::And(value),
        ['8', _, _, '3'] => OpCode::Xor(value),
        ['8', _, _, '4'] => OpCode::AddWithCarry(value),
        ['8', _, _, '5'] => OpCode::Sub(value),
        ['8', _, _, '6'] => OpCode::Shr(value),
        ['8', _, _, '7'] => OpCode::SubInverse(value),
        ['8', _, _, 'E'] => OpCode::Shl(value),
        _ => OpCode::Invalid,
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
    #[test]
    fn jmp_should_parse() {
        let opcode: u16 = 0x1200;
        assert_eq!(OpCode::Jump(opcode), opcode.into());
    }

    #[test]
    fn call_should_parse() {
        let opcode: u16 = 0x25E0;
        assert_eq!(OpCode::Call(opcode), opcode.into());
    }
    #[test]
    fn skip_value_should_parse() {
        let opcode: u16 = 0x35E0;
        assert_eq!(OpCode::SkipIfRegisterEqualsValue(opcode), opcode.into());
        let opcode: u16 = 0x45E0;
        assert_eq!(OpCode::SkipIfRegisterNotEqualsValue(opcode), opcode.into());
    }
    #[test]
    fn skip_register_should_parse() {
        let opcode: u16 = 0x55E0;
        assert_eq!(OpCode::SkipIfRegistersAreEqual(opcode), opcode.into());
        let opcode: u16 = 0x95E0;
        assert_eq!(OpCode::SkipIfRegistersAreNotEqual(opcode), opcode.into());
    }
    #[test]
    fn load_should_parse() {
        let opcode: u16 = 0x65E0;
        assert_eq!(OpCode::Load(opcode), opcode.into());
        let opcode: u16 = 0x85E0;
        assert_eq!(OpCode::LoadRegister(opcode), opcode.into());
    }
    #[test]
    fn add_should_parse() {
        let opcode: u16 = 0x75E0;
        assert_eq!(OpCode::Add(opcode), opcode.into());
        let opcode: u16 = 0x85E4;
        assert_eq!(OpCode::AddWithCarry(opcode), opcode.into());
    }
    #[test]
    fn or_should_parse() {
        let opcode: u16 = 0x85E1;
        assert_eq!(OpCode::Or(opcode), opcode.into());
    }
    #[test]
    fn and_should_parse() {
        let opcode: u16 = 0x85E2;
        assert_eq!(OpCode::And(opcode), opcode.into());
    }
    #[test]
    fn xor_should_parse() {
        let opcode: u16 = 0x85E3;
        assert_eq!(OpCode::Xor(opcode), opcode.into());
    }
    #[test]
    fn sub_should_parse() {
        let opcode: u16 = 0x85E5;
        assert_eq!(OpCode::Sub(opcode), opcode.into());
        let opcode: u16 = 0x85E7;
        assert_eq!(OpCode::SubInverse(opcode), opcode.into());
    }
    #[test]
    fn sh_should_parse() {
        let opcode: u16 = 0x85E6;
        assert_eq!(OpCode::Shr(opcode), opcode.into());
        let opcode: u16 = 0x85EE;
        assert_eq!(OpCode::Shl(opcode), opcode.into());
    }
}
