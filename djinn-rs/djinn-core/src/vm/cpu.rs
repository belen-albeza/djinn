use super::Result;
use crate::asm::Opcode;

mod stack;
use stack::Stack;

pub struct Cpu {
    pc: usize,
    stack: Stack,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0,
            stack: Stack::default(),
        }
    }

    /// Executes an opcode and returns whether the process has yielded.
    pub fn exec_opcode(&mut self, opcode: Opcode) -> Result<bool> {
        match opcode {
            Opcode::NoOp => Ok(false),
            Opcode::Yield => Ok(true),
        }
    }

    /// Reads the next opcode from the instruction slice.
    pub fn read_opcode(&mut self, instructions: &[Opcode]) -> Option<Opcode> {
        let opcode = instructions.get(self.pc)?;
        self.pc += 1;
        Some(*opcode)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
