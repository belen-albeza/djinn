use crate::asm::{Location, ProcessType};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RuntimeError {
    #[error("Invalid ROM")]
    InvalidRom,
    #[error("Stack underflow")]
    StackUnderflow(Location),
    #[error("Type error: {1}")]
    TypeError(Location, String),
    #[error("Division by zero")]
    DivisionByZero(Location),
    #[error("Process {0} not found")]
    ProcessNotFound(ProcessType),
}

impl RuntimeError {
    pub fn location(&self) -> Location {
        match self {
            RuntimeError::StackUnderflow(location) => *location,
            RuntimeError::TypeError(location, _) => *location,
            RuntimeError::DivisionByZero(location) => *location,
            RuntimeError::InvalidRom => Location::default(),
            RuntimeError::ProcessNotFound(_) => Location::default(),
        }
    }

    pub fn with_location(self, location: Location) -> Self {
        match self {
            RuntimeError::StackUnderflow(_) => RuntimeError::StackUnderflow(location),
            RuntimeError::TypeError(_, message) => RuntimeError::TypeError(location, message),
            RuntimeError::DivisionByZero(_) => RuntimeError::DivisionByZero(location),
            _ => self,
        }
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;
