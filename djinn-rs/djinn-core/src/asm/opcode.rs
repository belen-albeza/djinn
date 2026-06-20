use crate::asm::Value;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Opcode {
    #[default]
    NoOp,
    // Process control
    Yield,
    // Stack
    Push(Value),
    Pop,
    Dup,
    // ALU
    Not,
    And,
    Or,
    Xor,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
}
