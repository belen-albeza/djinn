use crate::asm::{Number, Value};

pub const BUILTIN_LOCALS: [(&str, Value); 4] = [
    ("x", Value::Numeric(Number::Float(0.0))),
    ("y", Value::Numeric(Number::Float(0.0))),
    ("z", Value::Numeric(Number::Int(0))),
    ("color", Value::Numeric(Number::Int(0))),
];
