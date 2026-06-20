mod cpu;
mod process;

use crate::asm::{Instruction, Location, ProcessId, ProcessType};
use process::{Process, Status};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RuntimeError {
    #[error("Invalid ROM")]
    LoadRomError,
    #[error("Stack underflow")]
    StackUnderflow(Location),
    #[error("Type error: {1}")]
    TypeError(Location, String),
    #[error("Division by zero")]
    DivisionByZero(Location),
}

impl RuntimeError {
    pub fn location(&self) -> Location {
        match self {
            RuntimeError::StackUnderflow(location) => *location,
            RuntimeError::TypeError(location, _) => *location,
            RuntimeError::DivisionByZero(location) => *location,
            RuntimeError::LoadRomError => Location::default(),
        }
    }

    pub fn with_location(self, location: Location) -> Self {
        match self {
            RuntimeError::StackUnderflow(_) => RuntimeError::StackUnderflow(location),
            RuntimeError::TypeError(_, message) => RuntimeError::TypeError(location, message),
            RuntimeError::DivisionByZero(_) => RuntimeError::DivisionByZero(location),
            RuntimeError::LoadRomError => RuntimeError::LoadRomError,
        }
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

pub trait Devices {
    fn video_buffer(&self) -> &[u8];
}

pub trait InstructionProvider {
    fn instructions(&self) -> &[Instruction];
}

pub struct Machine<D: Devices, R: InstructionProvider> {
    devices: D,
    rom: R,
    // TODO: use a proper process scheduler
    main_process: Process,
}

impl<D: Devices, R: InstructionProvider> Machine<D, R> {
    pub fn new(devices: D, rom: R) -> Self {
        Self {
            devices,
            rom,
            main_process: Process::new(ProcessId(1), ProcessType(0)),
        }
    }

    pub fn tick(&mut self) -> Result<bool> {
        self.main_process.tick(self.rom.instructions())?;
        let shall_halt = self.main_process.status() == Status::Terminated;

        Ok(shall_halt)
    }

    pub fn devices(&self) -> &D {
        &self.devices
    }
}
