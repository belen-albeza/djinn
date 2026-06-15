use std::fmt;

mod error;
mod token;

use djinn_core::cart::Rom;
pub use error::AssemblerError;

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

pub fn compile(_source_code: &str) -> Result<Rom> {
    // Ok(Rom::new(vec![Opcode::NoOp, Opcode::Yield, Opcode::NoOp]))
    Err(AssemblerError::LexerError {
        position: Location {
            line: 20,
            column: 1,
        },
        message: "Unexpected character `*`".to_string(),
    })
}
