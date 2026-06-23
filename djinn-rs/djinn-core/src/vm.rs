mod cpu;
mod process;
pub mod memory;

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

#[cfg_attr(test, mockall::automock)]
pub trait ProcessSignaler {
    fn spawn(&mut self, process_type: ProcessType) -> ProcessId;
    fn kill(&mut self, process_id: ProcessId);
}

#[cfg_attr(test, mockall::automock)]
pub trait Memory {
    fn poke(&mut self, id: ProcessId,address: usize, value: Value) -> Result<()>;
    fn peek(&self, id: ProcessId, address: usize) -> Result<Value>;
}

pub struct Machine<D: Devices, R: InstructionProvider, M: Memory> {
    devices: D,
    rom: R,
    processes: Vec<Process>,
    process_controller: Controller,
    locals: M,
}


impl<D: Devices, R: InstructionProvider, M: Memory> Machine<D, R, M> {
    pub fn new(devices: D, rom: R, locals: M) -> Self {
        let mut res = Self {
            devices,
            rom,
            processes: vec![],
            process_controller: Controller::new(),
            locals,
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
            locals: &mut self.locals,
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
        // drain and add spawned processes to the general process list
        self.process_controller
            .drain_spawned_into(&mut self.processes);

        // mark killed processes as terminated
        for id in self.process_controller.killed_mut().drain(..) {
            if let Some(p) = self.processes.iter_mut().find(|process| process.id() == id) {
                p.set_status(Status::Terminated);
            }
        }

        // remove terminated processes
        self.processes
            .retain(|process| process.status() != Status::Terminated);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::{Opcode, ProcessDefinition, ProcessType, Number};
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

    fn any_memory() -> impl Memory {
        let mut memory = MockMemory::new();
        memory.expect_poke().returning(|__, _, _| Ok(()));
        memory.expect_peek().returning(|__, _| Ok(Value::Numeric(Number::Int(0))));
        memory
    }

    fn any_machine_with_rom(
        r: impl InstructionProvider,
    ) -> Machine<impl Devices, impl InstructionProvider, impl Memory> {
        Machine::new(any_devices(), r, any_memory())
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

        let mut machine = Machine::new(devices, rom, any_memory());

        // Frame 1: only process #1 exists; it spawns #2 but it starts next frame.
        machine.tick().unwrap();
        assert_eq!(*trace.lock().unwrap(), vec![1]);

        // clear the trace
        trace.lock().unwrap().clear();

        // Frame 2: both process run, #1 before #2 (spawn order)
        machine.tick().unwrap();
        assert_eq!(*trace.lock().unwrap(), vec![1, 2]);
    }

    #[test]
    fn test_killed_processes_are_removed() {
        let rom = Rom::new(HashMap::from([
            (
                ProcessType(1),
                ProcessDefinition::new(
                    ProcessType(1),
                    vec![
                        Instruction::new(Opcode::Spawn(ProcessType(2)), Location::default()),
                        Instruction::new(Opcode::Yield, Location::default()),
                        Instruction::new(
                            Opcode::Push(Value::Process(ProcessId(2))),
                            Location::default(),
                        ),
                        Instruction::new(Opcode::Kill, Location::default()),
                    ],
                ),
            ),
            (
                ProcessType(2),
                ProcessDefinition::new(
                    ProcessType(2),
                    vec![Instruction::new(Opcode::Yield, Location::default())],
                ),
            ),
        ]));

        // Frame 1: process #1 spawns #2, then yields
        let mut machine = Machine::new(any_devices(), rom, any_memory());
        machine.tick().unwrap();
        assert_eq!(machine.processes.len(), 2);

        // Frame 2: process #1 kills #2, then terminates. Process #2 yields.
        machine.tick().unwrap();
        assert!(machine.processes.is_empty()); // no process remains
    }
}
