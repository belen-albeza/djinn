use std::fmt;

mod error;
mod lexer;
mod parser;
mod token;

use djinn_core::asm::Opcode;
use djinn_core::cart::Rom;
pub use error::{AssemblerError, Result};
use lexer::Lexer;
use parser::Parser;

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

pub fn compile(source_code: &str) -> Result<Rom> {
    let mut lexer = Lexer::new(source_code);
    let mut parser = Parser::new();

    // TODO: parse all processes instead
    let instructions = parser.parse_process(&mut lexer)?;

    Ok(Rom::new(instructions))
}
