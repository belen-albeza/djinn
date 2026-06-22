mod cpu;
mod process;

use crate::asm::{Instruction, ProcessId, ProcessType, Value};
use crate::devices::DeviceType;
use crate::error::{Result, RuntimeError};
use process::{Process, Status};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::{ProcessDefinition, ProcessType};
    use crate::cart::Rom;

    fn any_devices() -> impl Devices {
        let mut devices = MockDevices::new();
        devices.expect_call_api().returning(|_, _, _| Ok(false));
        devices.expect_video_buffer().return_const(Vec::<u8>::new());
        devices.expect_stdout().return_const(vec![]);
        devices
    }

    fn any_rom(extra_processes: Vec<(ProcessType, ProcessDefinition)>) -> Rom {
        let with_main = extra_processes.into_iter().chain(vec![(
            ProcessType(1),
            ProcessDefinition::new(ProcessType(1), vec![]),
        )]);

        Rom::new(with_main.into_iter().collect())
    }

    fn any_machine_with_rom(
        r: impl InstructionProvider,
    ) -> Machine<impl Devices, impl InstructionProvider> {
        Machine::new(any_devices(), r)
    }

    #[test]
    fn test_new_automatically_spawns_main_process() {
        let machine = any_machine_with_rom(any_rom(vec![]));
        assert_eq!(machine.processes.len(), 1);
        assert_eq!(machine.processes[0].process_type(), ProcessType(1));
    }

    #[test]
    fn test_spawn_process() {
        let rom = any_rom(vec![(
            ProcessType(2),
            ProcessDefinition::new(ProcessType(2), vec![]),
        )]);

        let mut machine = any_machine_with_rom(rom);
        machine.spawn_process(ProcessType(2));

        assert_eq!(machine.processes.len(), 2);
        assert_eq!(machine.processes[0].process_type(), ProcessType(1));
        assert_eq!(machine.processes[1].process_type(), ProcessType(2));
    }
}
