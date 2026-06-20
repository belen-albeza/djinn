#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Float(f64),
    Int(i32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Numeric(Number),
    Bool(bool),
}
