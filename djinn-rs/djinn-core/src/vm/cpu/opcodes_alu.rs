use super::Cpu;
use crate::asm::{Number, Value};
use crate::vm::{Result, RuntimeError};

impl Cpu {
    pub fn exec_opcode_not(&mut self) -> Result<bool> {
        let value = self.stack.pop()?;
        match value {
            // boolean NOT
            Value::Bool(value) => {
                self.stack.push(Value::Bool(!value));
            }
            // bitwise NOT
            Value::Numeric(Number::Int(value)) => {
                self.stack
                    .push(Value::Numeric(Number::Int(!(value as u32) as i32)));
            }
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "NOT opcode not supported for value: `{}`",
                    value
                )));
            }
        }
        Ok(false)
    }

    pub fn exec_opcode_and(&mut self) -> Result<bool> {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        match (a, b) {
            (Value::Bool(x), Value::Bool(y)) => {
                self.stack.push(Value::Bool(x && y));
            }
            (Value::Numeric(Number::Int(x)), Value::Numeric(Number::Int(y))) => {
                self.stack.push(Value::Numeric(Number::Int(x & y)));
            }
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "AND opcode not supported for values: `{}` and `{}`",
                    a, b
                )));
            }
        }
        Ok(false)
    }

    pub fn exec_opcode_or(&mut self) -> Result<bool> {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        match (a, b) {
            (Value::Bool(x), Value::Bool(y)) => {
                self.stack.push(Value::Bool(x || y));
            }
            (Value::Numeric(Number::Int(x)), Value::Numeric(Number::Int(y))) => {
                self.stack.push(Value::Numeric(Number::Int(x | y)));
            }
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "OR opcode not supported for values: `{}` and `{}`",
                    a, b
                )));
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        assert_eq!(cpu.exec_opcode_not(), Ok(false));
        assert_eq!(cpu.stack.pop(), Ok(Value::Bool(false)));

        cpu.stack
            .push(Value::Numeric(Number::Int(0x00_00_00_01_u32 as i32)));
        assert_eq!(cpu.exec_opcode_not(), Ok(false));
        assert_eq!(
            cpu.stack.pop(),
            Ok(Value::Numeric(Number::Int(0xFF_FF_FF_FE_u32 as i32)))
        );
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
        cpu.stack
            .push(Value::Numeric(Number::Int(0x00_00_00_01_u32 as i32)));
        cpu.stack
            .push(Value::Numeric(Number::Int(0x00_00_00_02_u32 as i32)));
        assert_eq!(cpu.exec_opcode_and(), Ok(false));
        assert_eq!(
            cpu.stack.pop(),
            Ok(Value::Numeric(Number::Int(0x00_00_00_00_u32 as i32)))
        );
    }

    #[test]
    fn test_and_opcode_with_mixed_values() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack
            .push(Value::Numeric(Number::Int(0x00_00_00_01_u32 as i32)));
        assert_eq!(
            cpu.exec_opcode_and(),
            Err(RuntimeError::TypeError(format!(
                "AND opcode not supported for values: `{}` and `{}`",
                Value::Bool(true),
                Value::Numeric(Number::Int(0x00_00_00_01_u32 as i32))
            )))
        );
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
