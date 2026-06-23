use crate::asm::{Location, ProcessId, ProcessType};

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
    #[error("Unknown local address ${2} for process {1}")]
    LocalNotFound(Location, ProcessId, usize),
}

impl RuntimeError {
    pub fn location(&self) -> Location {
        match self {
            RuntimeError::StackUnderflow(location) => *location,
            RuntimeError::TypeError(location, _) => *location,
            RuntimeError::DivisionByZero(location) => *location,
            RuntimeError::InvalidRom => Location::default(),
            RuntimeError::ProcessNotFound(_) => Location::default(),
            RuntimeError::LocalNotFound(location, _, _) => *location,
        }
    }

    pub fn with_location(self, location: Location) -> Self {
        match self {
            RuntimeError::StackUnderflow(_) => RuntimeError::StackUnderflow(location),
            RuntimeError::TypeError(_, message) => RuntimeError::TypeError(location, message),
            RuntimeError::DivisionByZero(_) => RuntimeError::DivisionByZero(location),
            RuntimeError::LocalNotFound(_, id, addr) => RuntimeError::LocalNotFound(location, id, addr),
            _ => self,
        }
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;
