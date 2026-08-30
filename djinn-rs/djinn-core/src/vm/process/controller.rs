use super::Process;
use crate::asm::{ProcessId, ProcessType};
use crate::error::Result;
use crate::vm::ProcessSignaler;
use crate::vm::RomProvider;
use std::rc::Rc;

/// Spawns / Kills processes
pub struct Controller<R: RomProvider> {
    rom: R,
    spawned: Vec<Process>,
    killed: Vec<ProcessId>,
    next_spawn_id: u32,
}

impl<R: RomProvider> Controller<R> {
    pub fn new(rom: R) -> Self {
        Self {
            rom,
            spawned: vec![],
            killed: vec![],
            next_spawn_id: 1,
        }
    }

    pub fn instantiate(&mut self, process_type: ProcessType) -> Result<ProcessId> {
        let id = ProcessId(self.next_spawn_id);
        self.next_spawn_id += 1;

        let code = self.rom.instructions(process_type)?;
        let process = Process::new(id, process_type, code);

        self.spawned.push(process);
        Ok(id)
    }

    // Cancel same-frame kills, then move survivors into `processes`,
    pub fn drain_spawned_into(&mut self, processes: &mut Vec<Process>) {
        let killed = &self.killed;
        self.spawned.retain(|p| !killed.contains(&p.id())); // drop cancelled spawns, in place
        processes.append(&mut self.spawned); // moves elements, keeps capacity
    }

    pub fn killed_mut(&mut self) -> &mut Vec<ProcessId> {
        &mut self.killed
    }
}

impl<R: RomProvider> ProcessSignaler for Controller<R> {
    fn spawn(&mut self, process_type: ProcessType) -> Result<(ProcessId, Rc<[usize]>)> {
        let id = self.instantiate(process_type)?;
        let args = self.rom.args(process_type)?;
        Ok((id, args))
    }

    fn kill(&mut self, process_id: ProcessId) {
        self.killed.push(process_id);
    }
}
