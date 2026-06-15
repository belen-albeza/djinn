use std::fmt;

use crate::asm::Location;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // 1-char tokens
    Colon,
    Tilde,
    // Multi-char tokens
    Id,
    // Opcodes
    NoOp,
    Yield,
    // Control
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
    pub location: Location,
}
