use crate::asm::Location;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AssemblerError {
    #[error("Unexpected character at {0}: {1}.")]
    UnexpectedCharacter(Location, char),
    #[error("Unexpected token at {location}: `{token}`. Expecting: {}.", .expected.iter().map(|e| format!("`{e}`")).collect::<Vec<String>>().join(", "))]
    UnexpectedToken {
        location: Location,
        token: String,
        expected: Vec<String>,
    },
    #[error("Main process not found.")]
    MainProcessNotFound(Location),
    #[error("Already defined process at {0}: {1}.")]
    ProcessAlreadyDefined(Location, String),
}

impl AssemblerError {
    pub fn location(&self) -> Location {
        match self {
            AssemblerError::UnexpectedCharacter(loc, _) => *loc,
            AssemblerError::UnexpectedToken { location, .. } => *location,
            AssemblerError::MainProcessNotFound(loc) => *loc,
            AssemblerError::ProcessAlreadyDefined(loc, _) => *loc,
        }
    }

    pub fn message(&self) -> String {
        format!("{}", self)
    }
}

pub type Result<T> = std::result::Result<T, AssemblerError>;
