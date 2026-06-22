use super::Process;
use crate::asm::{ProcessId, ProcessType};
use crate::vm::ProcessSignaler;

/// Spawns / Kills processes
pub struct Controller {
    spawned: Vec<Process>,
    killed: Vec<ProcessId>,
    next_spawn_id: u32,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            spawned: vec![],
            killed: vec![],
            next_spawn_id: 1,
        }
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

impl ProcessSignaler for Controller {
    fn spawn(&mut self, process_type: ProcessType) -> ProcessId {
        let id = ProcessId(self.next_spawn_id);
        let process = Process::new(id, process_type);
        self.spawned.push(process);
        self.next_spawn_id += 1;

        id
    }

    fn kill(&mut self, process_id: ProcessId) {
        self.killed.push(process_id);
    }
}
