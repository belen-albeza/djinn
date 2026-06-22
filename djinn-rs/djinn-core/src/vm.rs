mod cpu;
mod process;

use crate::asm::{Instruction, Location, ProcessId, ProcessType, Value};
use crate::devices::DeviceType;
use process::{Process, Status};

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
    fn stdout(&self) -> &[String];
    fn clear_stdout(&mut self);
}

#[cfg_attr(test, mockall::automock)]
pub trait Stacked {
    fn push_stack(&mut self, value: Value);
    fn pop_stack(&mut self) -> Result<Value>;
}

pub trait InstructionProvider {
    fn instructions(&self, process_type: ProcessType) -> Result<&[Instruction]>;
}

pub struct Machine<D: Devices, R: InstructionProvider> {
    devices: D,
    rom: R,
    processes: Vec<Process>,
    next_process_id: u32,
}

impl<D: Devices, R: InstructionProvider> Machine<D, R> {
    pub fn new(devices: D, rom: R) -> Self {
        let mut res = Self {
            devices,
            rom,
            processes: vec![],
            next_process_id: 1,
        };

        // spawn main process
        res.spawn_process(ProcessType(1));
        res
    }

    pub fn tick(&mut self) -> Result<bool> {
        self.devices.clear_stdout();

        for process in &mut self.processes {
            process.tick(
                &mut self.devices,
                self.rom.instructions(process.process_type())?,
            )?;
        }

        let shall_halt = self.processes.is_empty()
            || self
                .processes
                .iter()
                .all(|process| process.status() == Status::Terminated);

        Ok(shall_halt)
    }

    pub fn devices(&self) -> &D {
        &self.devices
    }

    fn spawn_process(&mut self, process_type: ProcessType) {
        self.processes
            .push(Process::new(ProcessId(self.next_process_id), process_type));
        self.next_process_id += 1;
    }
}
