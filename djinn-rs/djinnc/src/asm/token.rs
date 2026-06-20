use std::fmt;

use crate::asm::Location;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // 1-char tokens
    Colon,
    Tilde,
    Hash,
    // Multi-char tokens
    Id,
    Int(i32),
    Float(f64),
    Bool(bool),
    // Process control opcodes
    NoOp,
    Yield,
    // Stack opcodes
    Push,
    Pop,
    Dup,
    // ALU
    Not,
    And,
    Or,
    Xor,
    // Control
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::Hash => write!(f, "#"),
            TokenKind::Id => write!(f, "Identifier"),
            TokenKind::Yield => write!(f, "YLD"),
            TokenKind::Int(_) => write!(f, "Int"),
            TokenKind::Float(_) => write!(f, "Float"),
            TokenKind::Bool(_) => write!(f, "Bool"),
            _ => write!(f, "{}", format!("{self:?}").to_ascii_uppercase()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
    pub location: Location,
}
