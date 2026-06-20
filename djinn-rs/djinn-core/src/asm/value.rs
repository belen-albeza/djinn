use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Float(f64),
    Int(i32),
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
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(x) => *x,
            Value::Numeric(Number::Int(x)) => *x != 0,
            Value::Numeric(Number::Float(x)) => *x != 0.0,
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
