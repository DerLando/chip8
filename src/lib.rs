use opcode::OpCode;

mod cpu;
mod decode;
mod display;
mod emulator;
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
        let opcode = opcode.into();
        self.execute(opcode);
    }

    fn load_op(&mut self) -> u16 {
        let pc = self.cpu.pc_mut();
        let opcode = self.memory.load(*pc);
        *pc += 2;

        opcode
    }

    fn execute(&mut self, opcode: OpCode) {
        todo!()
    }
}
