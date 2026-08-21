use super::cpu::{Context, Cpu};
use crate::asm::{ProcessId, ProcessType};
use crate::vm::{Devices, Memory, ProcessSignaler, Result, RomProvider};

mod controller;
pub use controller::Controller;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Running,
    Terminated,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Running => write!(f, "running"),
            Status::Terminated => write!(f, "terminated"),
        }
    }
}

pub struct Process {
    id: ProcessId,
    process_type: ProcessType,
    status: Status,
    cpu: Cpu,
}

impl Process {
    pub fn new(id: ProcessId, process_type: ProcessType) -> Self {
        Self {
            cpu: Cpu::new(id),
            process_type,
            status: Status::Running,
            id,
        }
    }

    /// Runs process until it yields or terminates.
    pub fn tick<'a, D: Devices, S: ProcessSignaler, M: Memory, R: RomProvider>(
        &mut self,
        ctx: &mut Context<'a, D, S, M, R>,
    ) -> Result<()> {
        let instructions = ctx.rom.instructions(self.process_type)?;

        while let Some(instruction) = self.cpu.read_opcode(instructions) {
            let yielded = self.cpu.exec_opcode(ctx, instruction)?;
            if yielded {
                return Ok(());
            }
        }

        self.status = Status::Terminated;
        Ok(())
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    #[allow(unused)]
    pub fn process_type(&self) -> ProcessType {
        self.process_type
    }

    pub fn id(&self) -> ProcessId {
        self.id
    }
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Process #{} ({}): {}",
            self.id.0, self.process_type.0, self.status
        )
    }
}
