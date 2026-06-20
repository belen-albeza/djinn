use super::Cpu;
use crate::vm::Result;

impl Cpu {
    pub fn exec_opcode_not(&mut self) -> Result<bool> {
        let value = self.stack.pop()?;
        self.stack.push(value.not());
        Ok(false)
    }

    pub fn exec_opcode_and(&mut self) -> Result<bool> {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        self.stack.push(a.and(&b));
        Ok(false)
    }

    pub fn exec_opcode_or(&mut self) -> Result<bool> {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        self.stack.push(a.or(&b));
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::{Number, Value};
    use crate::vm::RuntimeError;

    #[test]
    fn test_not_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        assert_eq!(cpu.exec_opcode_not(), Ok(false));
        assert_eq!(cpu.stack.pop(), Ok(Value::Bool(false)));

        cpu.stack.push(Value::Numeric(Number::Int(1)));
        assert_eq!(cpu.exec_opcode_not(), Ok(false));
        assert_eq!(cpu.stack.pop(), Ok(Value::Bool(false)));
    }

    #[test]
    fn test_and_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack.push(Value::Bool(true));
        assert_eq!(cpu.exec_opcode_and(), Ok(false));
        assert_eq!(cpu.stack.pop(), Ok(Value::Bool(true)));
    }

    #[test]
    fn test_and_opcode_with_numeric_values() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        cpu.stack.push(Value::Numeric(Number::Float(0.0)));
        assert_eq!(cpu.exec_opcode_and(), Ok(false));
        assert_eq!(cpu.stack.pop(), Ok(Value::Bool(false)));
    }

    #[test]
    fn test_and_opcode_with_mixed_values() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        assert_eq!(cpu.exec_opcode_and(), Ok(false));
        assert_eq!(cpu.stack.pop(), Ok(Value::Bool(true)));
    }

    #[test]
    fn text_and_opcode_with_stack_underflow() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        assert_eq!(cpu.exec_opcode_and(), Err(RuntimeError::StackUnderflow));
    }

    #[test]
    fn test_or_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack.push(Value::Bool(true));
        assert_eq!(cpu.exec_opcode_or(), Ok(false));
        assert_eq!(cpu.stack.pop(), Ok(Value::Bool(true)));
    }
}
