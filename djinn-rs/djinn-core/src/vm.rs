mod cpu;
mod process;

use crate::asm::{Instruction, Location, ProcessType, Value};
use crate::devices::DeviceType;
use crate::error::{Result, RuntimeError};
use process::{Controller, Process, Status};

#[cfg_attr(test, mockall::automock)]
pub trait Devices {
    #[cfg_attr(test, mockall::concretize)]
    fn call_api<S: ValueStack>(
        &mut self,
        device_type: DeviceType,
        api_op: u8,
        stack: &mut S,
        location: Location,
    ) -> Result<bool>;
    fn video_buffer(&self) -> &[u8];
    fn stdout(&self) -> &[String];
    fn clear_stdout(&mut self);
}

#[cfg_attr(test, mockall::automock)]
pub trait ValueStack {
    fn push(&mut self, value: Value);
    fn pop(&mut self, location: Location) -> Result<Value>;
}

pub trait InstructionProvider {
    fn instructions(&self, process_type: ProcessType) -> Result<&[Instruction]>;
}

pub struct Machine<D: Devices, R: InstructionProvider> {
    devices: D,
    rom: R,
    processes: Vec<Process>,
    process_controller: Controller,
}

impl<D: Devices, R: InstructionProvider> Machine<D, R> {
    pub fn new(devices: D, rom: R) -> Self {
        let mut res = Self {
            devices,
            rom,
            processes: vec![],
            process_controller: Controller::new(),
        };

        // spawn main process
        res.process_controller.spawn(ProcessType(1));
        res.poll_process_controller();
        res
    }

    pub fn tick(&mut self) -> Result<bool> {
        self.devices.clear_stdout();

        // tick every running process
        for process in &mut self.processes {
            process.tick(
                &mut self.devices,
                self.rom.instructions(process.process_type())?,
            )?;
        }

        // check for newly spawned or killed processes
        self.poll_process_controller();

        // we halt if there are no processes or all processes are terminated
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

    fn poll_process_controller(&mut self) {
        // drain and add spawned processes to the general process list
        self.processes.append(self.process_controller.spawned_mut());
        // TODO: kill terminated processes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::{ProcessDefinition, ProcessType};
    use crate::cart::Rom;

    fn any_devices() -> impl Devices {
        let mut devices = MockDevices::new();
        devices.expect_call_api().returning(|_, _, _, _| Ok(false));
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
}
