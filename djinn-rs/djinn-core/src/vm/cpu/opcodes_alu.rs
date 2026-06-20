use std::ops::{Div, Rem};

use super::Cpu;
use crate::asm::{Number, Value};
use crate::vm::Result;

impl Cpu {
    pub fn exec_opcode_not(&mut self) -> Result<bool> {
        let value = self.pop_stack()?;
        self.stack.push(value.not());
        Ok(false)
    }

    pub fn exec_opcode_and(&mut self) -> Result<bool> {
        let b = self.pop_stack()?;
        let a = self.pop_stack()?;
        self.stack.push(a.and(&b));
        Ok(false)
    }

    pub fn exec_opcode_or(&mut self) -> Result<bool> {
        let b = self.pop_stack()?;
        let a = self.pop_stack()?;
        self.stack.push(a.or(&b));
        Ok(false)
    }

    pub fn exec_opcode_xor(&mut self) -> Result<bool> {
        let b = self.pop_stack()?;
        let a = self.pop_stack()?;
        self.stack.push(a.xor(&b));
        Ok(false)
    }

    pub fn exec_opcode_add(&mut self) -> Result<bool> {
        let b: Number = self.pop_stack()?.try_into()?;
        let a: Number = self.pop_stack()?.try_into()?;
        self.stack.push(Value::Numeric(a + b));
        Ok(false)
    }

    pub fn exec_opcode_sub(&mut self) -> Result<bool> {
        let b: Number = self.pop_stack()?.try_into()?;
        let a: Number = self.pop_stack()?.try_into()?;
        self.stack.push(Value::Numeric(a - b));
        Ok(false)
    }

    pub fn exec_opcode_mul(&mut self) -> Result<bool> {
        let b: Number = self.pop_stack()?.try_into()?;
        let a: Number = self.pop_stack()?.try_into()?;
        self.stack.push(Value::Numeric(a * b));
        Ok(false)
    }

    pub fn exec_opcode_div(&mut self) -> Result<bool> {
        let b: Number = self.pop_stack()?.try_into()?;
        let a: Number = self.pop_stack()?.try_into()?;
        self.stack.push(Value::Numeric(a.div(b)?));
        Ok(false)
    }

    pub fn exec_opcode_rem(&mut self) -> Result<bool> {
        let b: Number = self.pop_stack()?.try_into()?;
        let a: Number = self.pop_stack()?.try_into()?;
        self.stack.push(Value::Numeric(a.rem(b)?));
        Ok(false)
    }

    pub fn exec_opcode_inc(&mut self) -> Result<bool> {
        let value: Number = self.pop_stack()?.try_into()?;
        self.stack.push(Value::Numeric(value + Number::Int(1)));
        Ok(false)
    }

    pub fn exec_opcode_dec(&mut self) -> Result<bool> {
        let value: Number = self.pop_stack()?.try_into()?;
        self.stack.push(Value::Numeric(value - Number::Int(1)));
        Ok(false)
    }

    pub fn exec_opcode_eq(&mut self) -> Result<bool> {
        let b = self.pop_stack()?;
        let a = self.pop_stack()?;
        self.stack.push(Value::Bool(a == b));
        Ok(false)
    }

    pub fn exec_opcode_neq(&mut self) -> Result<bool> {
        let b = self.pop_stack()?;
        let a = self.pop_stack()?;
        self.stack.push(Value::Bool(a != b));
        Ok(false)
    }

    pub fn exec_opcode_lt(&mut self) -> Result<bool> {
        let b: Number = self.pop_stack()?.try_into()?;
        let a: Number = self.pop_stack()?.try_into()?;
        self.stack.push(Value::Bool(a < b));
        Ok(false)
    }

    pub fn exec_opcode_leq(&mut self) -> Result<bool> {
        let b: Number = self.pop_stack()?.try_into()?;
        let a: Number = self.pop_stack()?.try_into()?;
        self.stack.push(Value::Bool(a <= b));
        Ok(false)
    }

    pub fn exec_opcode_gt(&mut self) -> Result<bool> {
        let b: Number = self.pop_stack()?.try_into()?;
        let a: Number = self.pop_stack()?.try_into()?;
        self.stack.push(Value::Bool(a > b));
        Ok(false)
    }

    pub fn exec_opcode_geq(&mut self) -> Result<bool> {
        let b: Number = self.pop_stack()?.try_into()?;
        let a: Number = self.pop_stack()?.try_into()?;
        self.stack.push(Value::Bool(a >= b));
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::{Location, Number, Value};
    use crate::vm::RuntimeError;

    #[test]
    fn test_not_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        assert_eq!(cpu.exec_opcode_not(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(false)));

        cpu.stack.push(Value::Numeric(Number::Int(1)));
        assert_eq!(cpu.exec_opcode_not(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(false)));
    }

    #[test]
    fn test_and_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack.push(Value::Bool(true));
        assert_eq!(cpu.exec_opcode_and(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(true)));
    }

    #[test]
    fn test_and_opcode_with_numeric_values() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        cpu.stack.push(Value::Numeric(Number::Float(0.0)));
        assert_eq!(cpu.exec_opcode_and(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(false)));
    }

    #[test]
    fn test_and_opcode_with_mixed_values() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        assert_eq!(cpu.exec_opcode_and(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(true)));
    }

    #[test]
    fn test_and_opcode_with_stack_underflow() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        assert_eq!(
            cpu.exec_opcode_and(),
            Err(RuntimeError::StackUnderflow(Location::default()))
        );
    }

    #[test]
    fn test_or_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack.push(Value::Bool(true));
        assert_eq!(cpu.exec_opcode_or(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(true)));
    }

    #[test]
    fn test_xor_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack.push(Value::Bool(true));
        assert_eq!(cpu.exec_opcode_xor(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(false)));
    }

    #[test]
    fn test_add_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        cpu.stack.push(Value::Numeric(Number::Float(2.0)));
        assert_eq!(cpu.exec_opcode_add(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Numeric(Number::Float(3.0))));
    }

    #[test]
    fn test_add_opcode_returns_type_error() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        assert_eq!(
            cpu.exec_opcode_add(),
            Err(RuntimeError::TypeError(
                "`true` is not a number".to_string()
            ))
        );
    }

    #[test]
    fn test_sub_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(-1)));
        cpu.stack.push(Value::Numeric(Number::Float(2.0)));
        assert_eq!(cpu.exec_opcode_sub(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Numeric(Number::Float(-3.0))));
    }

    #[test]
    fn test_mul_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(2)));
        cpu.stack.push(Value::Numeric(Number::Int(3)));
        assert_eq!(cpu.exec_opcode_mul(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Numeric(Number::Int(6))));
    }

    #[test]
    fn test_div_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(6)));
        cpu.stack.push(Value::Numeric(Number::Int(2)));
        assert_eq!(cpu.exec_opcode_div(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Numeric(Number::Int(3))));
    }

    #[test]
    fn test_div_opcode_returns_division_by_zero_error() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Float(1.0)));
        cpu.stack.push(Value::Numeric(Number::Int(0)));
        assert_eq!(cpu.exec_opcode_div(), Err(RuntimeError::DivisionByZero));
    }

    #[test]
    fn test_rem_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(6)));
        cpu.stack.push(Value::Numeric(Number::Int(2)));
        assert_eq!(cpu.exec_opcode_rem(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Numeric(Number::Int(0))));
    }

    #[test]
    fn test_rem_opcode_returns_division_by_zero_error() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Float(1.0)));
        cpu.stack.push(Value::Numeric(Number::Int(0)));
        assert_eq!(cpu.exec_opcode_rem(), Err(RuntimeError::DivisionByZero));
    }

    #[test]
    fn test_inc_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        assert_eq!(cpu.exec_opcode_inc(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Numeric(Number::Int(2))));
    }

    #[test]
    fn test_dec_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        assert_eq!(cpu.exec_opcode_dec(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Numeric(Number::Int(0))));
    }

    #[test]
    fn test_eq_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack.push(Value::Bool(true));
        assert_eq!(cpu.exec_opcode_eq(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(true)));
    }

    #[test]
    fn test_eq_opcode_with_mixed_types() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        assert_eq!(cpu.exec_opcode_eq(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(false)));
    }

    #[test]
    fn test_eq_opcode_casts_ints_to_floats() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        cpu.stack.push(Value::Numeric(Number::Float(1.0)));
        assert_eq!(cpu.exec_opcode_eq(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(true)));
    }

    #[test]
    fn test_neq_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Bool(true));
        cpu.stack.push(Value::Bool(true));
        assert_eq!(cpu.exec_opcode_neq(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(false)));
    }

    #[test]
    fn test_lt_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        cpu.stack.push(Value::Numeric(Number::Int(2)));
        assert_eq!(cpu.exec_opcode_lt(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(true)));
    }

    #[test]
    fn test_leq_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(2)));
        cpu.stack.push(Value::Numeric(Number::Int(2)));
        assert_eq!(cpu.exec_opcode_leq(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(true)));
    }

    #[test]
    fn test_gt_opcode() {
        let mut cpu = Cpu::new();
        cpu.stack.push(Value::Numeric(Number::Int(2)));
        cpu.stack.push(Value::Numeric(Number::Int(1)));
        assert_eq!(cpu.exec_opcode_gt(), Ok(false));
        assert_eq!(cpu.pop_stack(), Ok(Value::Bool(true)));
    }
}
