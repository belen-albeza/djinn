mod cpu;
mod process;

use crate::asm::{Opcode, ProcessId, ProcessType};
use process::{Process, Status};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RuntimeError {
    #[error("Invalid ROM")]
    LoadRomError,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Type error: {0}")]
    TypeError(String),
}

type Result<T> = std::result::Result<T, RuntimeError>;

pub trait Devices {
    fn video_buffer(&self) -> &[u8];
}

pub trait InstructionProvider {
    fn instructions(&self) -> &[Opcode];
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
