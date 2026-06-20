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
}
