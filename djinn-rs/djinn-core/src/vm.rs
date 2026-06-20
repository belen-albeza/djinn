mod cpu;
mod process;

use crate::asm::{Instruction, Location, ProcessId, ProcessType, Value};
use crate::devices::DeviceType;
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

#[cfg_attr(test, mockall::automock)]
pub trait Devices {
    #[cfg_attr(test, mockall::concretize)]
    fn call_api<S: Stacked>(
        &mut self,
        device_type: DeviceType,
        api_op: u8,
        cpu: &mut S,
    ) -> Result<bool>;
    fn video_buffer(&self) -> &[u8];
    fn stdout(&self) -> String;
}

#[cfg_attr(test, mockall::automock)]
pub trait Stacked {
    fn push_stack(&mut self, value: Value);
    fn pop_stack(&mut self) -> Result<Value>;
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
        self.main_process
            .tick(&mut self.devices, self.rom.instructions())?;
        let shall_halt = self.main_process.status() == Status::Terminated;

        Ok(shall_halt)
    }

    pub fn devices(&self) -> &D {
        &self.devices
    }
}
