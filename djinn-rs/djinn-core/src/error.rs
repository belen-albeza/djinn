use crate::asm::{Location, ProcessId, ProcessType};
use crate::devices::DeviceType;

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
    #[error("Unknown global address {0}")]
    GlobalNotFound(Location, usize),
    #[error("Invalid API code {1} for device {2:?}")]
    InvalidApiCode(Location, u8, DeviceType),
    #[error("Invalid device type {1}")]
    InvalidDeviceType(Location, u8),
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
            RuntimeError::GlobalNotFound(location, _) => *location,
            RuntimeError::InvalidApiCode(location, _, _) => *location,
            RuntimeError::InvalidDeviceType(location, _) => *location,
        }
    }

    pub fn with_location(self, location: Location) -> Self {
        match self {
            RuntimeError::StackUnderflow(_) => RuntimeError::StackUnderflow(location),
            RuntimeError::TypeError(_, message) => RuntimeError::TypeError(location, message),
            RuntimeError::DivisionByZero(_) => RuntimeError::DivisionByZero(location),
            RuntimeError::LocalNotFound(_, id, addr) => {
                RuntimeError::LocalNotFound(location, id, addr)
            }
            RuntimeError::GlobalNotFound(_, addr) => RuntimeError::GlobalNotFound(location, addr),
            RuntimeError::InvalidApiCode(_, api_code, device_type) => {
                RuntimeError::InvalidApiCode(location, api_code, device_type)
            }
            RuntimeError::InvalidDeviceType(_, device_type) => {
                RuntimeError::InvalidDeviceType(location, device_type)
            }
            _ => self,
        }
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;
