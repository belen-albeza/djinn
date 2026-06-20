use crate::asm::Opcode;
use crate::vm::Result;

mod stack;
use stack::Stack;
mod opcodes_alu;

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
            Opcode::Push(value) => {
                self.stack.push(value);
                Ok(false)
            }
            Opcode::Pop => {
                self.stack.pop()?;
                Ok(false)
            }
            Opcode::Dup => {
                let value = self.stack.pop()?;
                self.stack.push(value);
                self.stack.push(value);
                Ok(false)
            }
            Opcode::Not => self.exec_opcode_not(),
            Opcode::And => self.exec_opcode_and(),
            Opcode::Or => self.exec_opcode_or(),
            Opcode::Xor => self.exec_opcode_xor(),
            Opcode::Add => self.exec_opcode_add(),
            Opcode::Sub => self.exec_opcode_sub(),
            Opcode::Mul => self.exec_opcode_mul(),
            Opcode::Div => self.exec_opcode_div(),
            Opcode::Mod => self.exec_opcode_rem(),
            Opcode::Eq => self.exec_opcode_eq(),
            Opcode::Neq => self.exec_opcode_neq(),
            Opcode::Lth => self.exec_opcode_lth(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::{Number, Value};

    fn any_cpu() -> Cpu {
        Cpu::default()
    }

    #[test]
    fn test_yield_opcode() {
        let mut cpu = any_cpu();
        assert_eq!(cpu.exec_opcode(Opcode::Yield), Ok(true));
    }

    #[test]
    fn test_noop_opcode() {
        let mut cpu = any_cpu();
        assert_eq!(cpu.exec_opcode(Opcode::NoOp), Ok(false));
    }

    #[test]
    fn test_push_opcode() {
        let mut cpu = any_cpu();
        assert_eq!(
            cpu.exec_opcode(Opcode::Push(Value::Numeric(Number::Int(1)))),
            Ok(false)
        );
        assert_eq!(cpu.stack.pop(), Ok(Value::Numeric(Number::Int(1))));
    }

    #[test]
    fn test_pop_opcode() {
        let mut cpu = any_cpu();
        cpu.stack.push(Value::Numeric(Number::Int(1)));

        assert_eq!(cpu.exec_opcode(Opcode::Pop), Ok(false));
        assert!(cpu.stack.is_empty());
    }

    #[test]
    fn test_dup_opcode() {
        let mut cpu = any_cpu();
        cpu.stack.push(Value::Numeric(Number::Int(1)));

        assert_eq!(cpu.exec_opcode(Opcode::Dup), Ok(false));
        assert_eq!(cpu.stack.pop(), Ok(Value::Numeric(Number::Int(1))));
        assert_eq!(cpu.stack.pop(), Ok(Value::Numeric(Number::Int(1))));
    }
}
