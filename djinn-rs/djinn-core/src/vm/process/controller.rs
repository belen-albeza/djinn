use super::Process;
use crate::asm::{ProcessId, ProcessType};
use crate::vm::ProcessSignaler;

/// Spawns / Kills processes
pub struct Controller {
    spawned: Vec<Process>,
    next_spawn_id: u32,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            spawned: vec![],
            next_spawn_id: 1,
        }
    }

    pub fn spawned_mut(&mut self) -> &mut Vec<Process> {
        &mut self.spawned
    }
}

impl ProcessSignaler for Controller {
    fn spawn(&mut self, process_type: ProcessType) -> ProcessId {
        let id = ProcessId(self.next_spawn_id);
        let process = Process::new(id, process_type);
        self.spawned.push(process);
        self.next_spawn_id += 1;

        id
    }
}
