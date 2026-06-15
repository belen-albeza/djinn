use crate::asm::Location;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AssemblerError {
    #[error("Unexpected character at {0}: {1}")]
    UnexpectedCharacter(Location, char),
    #[error("Unexpected token at {0}: {1}")]
    UnexpectedToken(Location, String),
}

impl AssemblerError {
    pub fn location(&self) -> Location {
        match self {
            AssemblerError::UnexpectedCharacter(loc, _) => *loc,
            AssemblerError::UnexpectedToken(loc, _) => *loc,
        }
    }
    pub fn message(&self) -> String {
        match self {
            AssemblerError::UnexpectedCharacter(_, c) => format!("Unexpected character `{}`", c),
            AssemblerError::UnexpectedToken(_, token) => format!("Unexpected token `{}`", token),
        }
    }
}

pub type Result<T> = std::result::Result<T, AssemblerError>;
