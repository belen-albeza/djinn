use std::fmt;

mod error;
mod lexer;
mod token;

use djinn_core::asm::Opcode;
use djinn_core::cart::Rom;
pub use error::AssemblerError;
use lexer::Lexer;
use token::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ln {}, Col {}", self.line, self.column)
    }
}

pub type Result<T> = std::result::Result<T, AssemblerError>;

pub fn compile(source_code: &str) -> Result<Rom> {
    let mut lexer = Lexer::new(source_code);
    let mut tokens = Vec::new();

    // TODO: do this in a parser
    loop {
        let token = lexer.scan_token()?;
        let eof = token.kind == TokenKind::Eof;
        tokens.push(token);
        if eof {
            break;
        }
    }

    let instructions: Vec<Opcode> = tokens
        .into_iter()
        .map(|token| match token.kind {
            TokenKind::NoOp => Ok(Some(Opcode::NoOp)),
            TokenKind::Yield => Ok(Some(Opcode::Yield)),
            TokenKind::Eof => Ok(None),
            _ => Err(AssemblerError::Parser(
                token.location,
                format!("Unexpected token `{}`", token.lexeme),
            )),
        })
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(Rom::new(instructions))
}
