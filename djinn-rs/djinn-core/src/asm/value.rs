use crate::asm::{Location, ProcessId};
use crate::error::{Result, RuntimeError};
use std::cmp::Ordering;
use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Float(f64),
    Int(i32),
}

impl Number {
    pub fn is_zero(&self) -> bool {
        match self {
            Number::Float(x) => *x == 0.0,
            Number::Int(x) => *x == 0,
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::Float(x), Number::Float(y)) => x == y,
            (Number::Int(x), Number::Int(y)) => x == y,
            (Number::Float(x), Number::Int(y)) => *x == (*y as f64),
            (Number::Int(x), Number::Float(y)) => (*x as f64) == *y,
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Number::Float(x), Number::Float(y)) => x.partial_cmp(y),
            (Number::Int(x), Number::Int(y)) => x.partial_cmp(y),
            (Number::Float(x), Number::Int(y)) => x.partial_cmp(&(*y as f64)),
            (Number::Int(x), Number::Float(y)) => (*x as f64).partial_cmp(y),
        }
    }
}

impl ops::Add for Number {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Float(x), Number::Float(y)) => Number::Float(x + y),
            (Number::Int(x), Number::Int(y)) => Number::Int(x.wrapping_add(y)),
            (Number::Float(x), Number::Int(y)) => Number::Float(x + y as f64),
            (Number::Int(x), Number::Float(y)) => Number::Float(x as f64 + y),
        }
    }
}

impl ops::Sub for Number {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Float(x), Number::Float(y)) => Number::Float(x - y),
            (Number::Int(x), Number::Int(y)) => Number::Int(x.wrapping_sub(y)),
            (Number::Float(x), Number::Int(y)) => Number::Float(x - y as f64),
            (Number::Int(x), Number::Float(y)) => Number::Float(x as f64 - y),
        }
    }
}

impl ops::Mul for Number {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Float(x), Number::Float(y)) => Number::Float(x * y),
            (Number::Int(x), Number::Int(y)) => Number::Int(x.wrapping_mul(y)),
            (Number::Float(x), Number::Int(y)) => Number::Float(x * y as f64),
            (Number::Int(x), Number::Float(y)) => Number::Float(x as f64 * y),
        }
    }
}

impl ops::Div for Number {
    type Output = Result<Self>;
    fn div(self, other: Self) -> Self::Output {
        if other.is_zero() {
            return Err(RuntimeError::DivisionByZero(Location::default()));
        }
        match (self, other) {
            (Number::Float(x), Number::Float(y)) => Ok(Number::Float(x / y)),
            (Number::Int(x), Number::Int(y)) => Ok(Number::Int(x.wrapping_div(y))),
            (Number::Float(x), Number::Int(y)) => Ok(Number::Float(x / y as f64)),
            (Number::Int(x), Number::Float(y)) => Ok(Number::Float(x as f64 / y)),
        }
    }
}

impl ops::Rem for Number {
    type Output = Result<Self>;
    fn rem(self, other: Self) -> Self::Output {
        if other.is_zero() {
            return Err(RuntimeError::DivisionByZero(Location::default()));
        }
        match (self, other) {
            (Number::Float(x), Number::Float(y)) => Ok(Number::Float(x.rem_euclid(y))),
            (Number::Int(x), Number::Int(y)) => Ok(Number::Int(x.wrapping_rem_euclid(y))),
            (Number::Float(x), Number::Int(y)) => Ok(Number::Float(x.rem_euclid(y as f64))),
            (Number::Int(x), Number::Float(y)) => Ok(Number::Float((x as f64).rem_euclid(y))),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Float(float) => write!(f, "{}", fmt_f64_min_1_decimal(*float)),
            Number::Int(int) => write!(f, "{}", int),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Numeric(Number),
    Bool(bool),
    Process(ProcessId),
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(x) => *x,
            Value::Numeric(Number::Int(x)) => *x != 0,
            Value::Numeric(Number::Float(x)) => *x != 0.0,
            Value::Process(x) => *x != ProcessId(0),
        }
    }

    pub fn and(&self, other: &Self) -> Self {
        Self::Bool(self.as_bool() && other.as_bool())
    }

    pub fn or(&self, other: &Self) -> Self {
        Self::Bool(self.as_bool() || other.as_bool())
    }

    pub fn not(&self) -> Self {
        Self::Bool(!self.as_bool())
    }

    pub fn xor(&self, other: &Self) -> Self {
        let a = self.as_bool();
        let b = other.as_bool();
        Self::Bool((a || b) && !(a && b))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Numeric(number) => write!(f, "{}", number),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Process(process_id) => write!(f, "{}", process_id),
        }
    }
}

impl TryFrom<Value> for Number {
    type Error = RuntimeError;
    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Numeric(number) => Ok(number),
            _ => Err(RuntimeError::TypeError(
                Location::default(),
                format!("`{}` is not a number", value),
            )),
        }
    }
}

impl TryFrom<Value> for ProcessId {
    type Error = RuntimeError;
    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Process(process_id) => Ok(process_id),
            _ => Err(RuntimeError::TypeError(
                Location::default(),
                format!("`{}` is not a process id", value),
            )),
        }
    }
}

fn fmt_f64_min_1_decimal(x: f64) -> String {
    let s = format!("{}", x);
    if s.contains('.') || s.contains('e') || s.contains('E') {
        s
    } else {
        format!("{s}.0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_display() {
        assert_eq!(Value::Numeric(Number::Int(1)).to_string(), "1");
        assert_eq!(Value::Bool(true).to_string(), "true");
        assert_eq!(Value::Numeric(Number::Float(1.0)).to_string(), "1.0");
    }
}
