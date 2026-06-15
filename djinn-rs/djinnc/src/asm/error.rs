use crate::asm::Location;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AssemblerError {
    #[error("Unexpected character at {0}: {1}")]
    UnexpectedCharacter(Location, char),
    #[error("Unexpected token at {0}: {1}")]
    UnexpectedToken(Location, String),
    #[error("No main process found")]
    NoMainProcessFound(Location),
}

impl AssemblerError {
    pub fn location(&self) -> Location {
        match self {
            AssemblerError::UnexpectedCharacter(loc, _) => *loc,
            AssemblerError::UnexpectedToken(loc, _) => *loc,
            AssemblerError::NoMainProcessFound(loc) => *loc,
        }
    }

    pub fn message(&self) -> String {
        format!("{}", self)
    }
}

pub type Result<T> = std::result::Result<T, AssemblerError>;
