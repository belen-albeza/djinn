mod cpu;
mod process;

use crate::asm::{Instruction, Location, ProcessId, ProcessType, Value};
use crate::devices::DeviceType;
use crate::error::{Result, RuntimeError};
use cpu::Context;
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

#[cfg_attr(test, mockall::automock)]
pub trait ProcessSignaler {
    fn spawn(&mut self, process_type: ProcessType) -> ProcessId;
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
        let mut ctx = Context {
            devices: &mut self.devices,
            signaler: &mut self.process_controller,
        };

        ctx.devices.clear_stdout();

        // tick every running process
        for process in &mut self.processes {
            if process.status() == Status::Terminated {
                continue;
            }

            process.tick(&mut ctx, self.rom.instructions(process.process_type())?)?;
        }

        // check for newly spawned or killed processes
        self.poll_process_controller();
        // we halt if there are no processes left
        let shall_halt = self.processes.is_empty();

        Ok(shall_halt)
    }

    pub fn devices(&self) -> &D {
        &self.devices
    }

    fn poll_process_controller(&mut self) {
        // remove terminated processes
        self.processes
            .retain(|process| process.status() != Status::Terminated);

        // drain and add spawned processes to the general process list
        self.processes.append(self.process_controller.spawned_mut());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::{Opcode, ProcessDefinition, ProcessType};
    use crate::cart::Rom;
    use std::collections::HashMap;

    fn any_devices() -> impl Devices {
        let mut devices = MockDevices::new();
        devices.expect_call_api().returning(|_, _, _, _| Ok(false));
        devices.expect_video_buffer().return_const(Vec::<u8>::new());
        devices.expect_stdout().return_const(vec![]);
        devices.expect_clear_stdout().returning(|| ());
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
    fn test_tick_spawns_new_processes() {
        let rom = Rom::new(HashMap::from([
            (
                ProcessType(1),
                ProcessDefinition::new(
                    ProcessType(1),
                    vec![
                        Instruction::new(Opcode::Spawn(ProcessType(2)), Location::default()),
                        Instruction::new(Opcode::Yield, Location::default()),
                    ],
                ),
            ),
            (
                ProcessType(2),
                ProcessDefinition::new(ProcessType(2), vec![]),
            ),
        ]));

        let mut machine = any_machine_with_rom(rom);
        assert_eq!(machine.processes.len(), 1);
        assert_eq!(machine.processes[0].process_type(), ProcessType(1));

        assert_eq!(machine.tick(), Ok(false));

        assert_eq!(machine.processes.len(), 2);
        assert_eq!(machine.processes[1].process_type(), ProcessType(2));
        assert_eq!(machine.process_controller.spawned_mut().len(), 0);

        assert_eq!(machine.tick(), Ok(true));
        assert_eq!(machine.processes.len(), 0);
    }

    #[test]
    fn test_concurrent_processes() {
        use crate::devices::DeviceType;
        use std::sync::{Arc, Mutex};

        // This tests hijacks the Device api to insert a mock that keeps track
        // of the order in which the processes ticked.
        let trace = Arc::new(Mutex::new(Vec::<u8>::new()));

        // Re-usable opcodes for convenience
        let probe =
            |id: u8| Instruction::new(Opcode::Device(DeviceType::Console, id), Location::default());
        let yield_ = || Instruction::new(Opcode::Yield, Location::default());
        let spawn = |t| Instruction::new(Opcode::Spawn(t), Location::default());

        let rom = Rom::new(HashMap::from([
            (
                ProcessType(1),
                ProcessDefinition::new(
                    ProcessType(1),
                    vec![
                        spawn(ProcessType(2)),
                        probe(1),
                        yield_(),
                        probe(1),
                        yield_(),
                    ],
                ),
            ),
            (
                ProcessType(2),
                ProcessDefinition::new(
                    ProcessType(2),
                    vec![probe(2), yield_(), probe(2), yield_()],
                ),
            ),
        ]));

        // Custom devices that record the probe id instead of any_devices().
        let mut devices = MockDevices::new();
        let log = trace.clone();
        devices.expect_call_api().returning(move |_, api_op, _, _| {
            log.lock().unwrap().push(api_op);
            Ok(false)
        });
        devices.expect_video_buffer().return_const(Vec::<u8>::new());
        devices.expect_stdout().return_const(vec![]);
        devices.expect_clear_stdout().returning(|| ());

        let mut machine = Machine::new(devices, rom);

        // Frame 1: only process #1 exists; it spawns #2 but it starts next frame.
        machine.tick().unwrap();
        assert_eq!(*trace.lock().unwrap(), vec![1]);

        // clear the trace
        trace.lock().unwrap().clear();

        // Frame 2: both process run, #1 before #2 (spawn order)
        machine.tick().unwrap();
        assert_eq!(*trace.lock().unwrap(), vec![1, 2]);
    }
}
