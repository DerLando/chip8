use opcode::OpCode;

mod cpu;
mod decode;
mod display;
mod interpret;
mod memory;
mod opcode;

/// The main emulator
pub struct Emulator {
    cpu: cpu::Cpu,
    memory: memory::Memory,
    stack: memory::Stack,
}

impl Emulator {
    pub fn tick(&mut self) {
        let opcode = self.load_op();
        let opcode = self.decode(opcode);
        self.execute(opcode);
    }

    fn load_op(&mut self) -> u16 {
        todo!()
    }

    fn decode(&self, opcode: u16) -> OpCode {
        todo!()
    }

    fn execute(&mut self, opcode: OpCode) {
        todo!()
    }
}
