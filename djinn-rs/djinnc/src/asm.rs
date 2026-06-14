use std::fmt;

use djinn_core::cart::Rom;

mod token;

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

#[derive(Debug, thiserror::Error)]
pub enum AssemblerError {
    #[error("Lexer error at {position}: {message}")]
    LexerError { position: Location, message: String },
}

impl AssemblerError {
    pub fn location(&self) -> Location {
        match self {
            AssemblerError::LexerError { position, .. } => *position,
        }
    }
    pub fn message(&self) -> String {
        match self {
            AssemblerError::LexerError { message, .. } => message.to_string(),
        }
    }
}

type Result<T> = std::result::Result<T, AssemblerError>;

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
