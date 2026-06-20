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
        match self {
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::Id => write!(f, "Identifier"),
            TokenKind::Yield => write!(f, "YLD"),
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
