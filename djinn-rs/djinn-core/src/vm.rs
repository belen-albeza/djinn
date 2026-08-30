use std::rc::Rc;

mod cpu;
pub mod memory;
mod process;

use crate::asm::{BUILTIN_LOCALS, Instruction, Location, ProcessId, ProcessType, Value};
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

pub trait RomProvider {
    fn instructions(&self, process_type: ProcessType) -> Result<Rc<[Instruction]>>;
    fn args(&self, process_type: ProcessType) -> Result<Rc<[usize]>>;
}

#[cfg_attr(test, mockall::automock)]
pub trait ProcessSignaler {
    fn spawn(&mut self, process_type: ProcessType) -> Result<(ProcessId, Rc<[usize]>)>;
    fn kill(&mut self, process_id: ProcessId);
}

#[cfg_attr(test, mockall::automock)]
pub trait LocalMemory {
    fn poke(&mut self, id: ProcessId, address: usize, value: Value) -> Result<()>;
    fn peek(&self, id: ProcessId, address: usize) -> Result<Value>;
    fn free(&mut self, id: ProcessId);
}

#[cfg_attr(test, mockall::automock)]
pub trait GlobalMemory {
    fn poke(&mut self, address: usize, value: Value) -> Result<()>;
    fn peek(&self, address: usize) -> Result<Value>;
}

pub struct Machine<D: Devices, R: RomProvider, L: LocalMemory, G: GlobalMemory> {
    devices: D,
    processes: Vec<Process>,
    process_controller: Controller<R>,
    locals: L,
    globals: G,
}

impl<D: Devices, R: RomProvider, L: LocalMemory, G: GlobalMemory> Machine<D, R, L, G> {
    pub fn new(devices: D, rom: R, locals: L, globals: G) -> Self {
        let mut res = Self {
            devices,
            processes: vec![],
            process_controller: Controller::new(rom),
            locals,
            globals,
        };

        // spawn main process
        res.process_controller
            .spawn(ProcessType(1))
            .expect("failed to spawn main process");
        // initialize builtin locals for main process
        for (i, (_, value)) in BUILTIN_LOCALS.iter().enumerate() {
            res.locals
                .poke(ProcessId(1), i, *value)
                .expect("failed to initialize builtin locals for main");
        }
        res.poll_process_controller();
        res
    }

    pub fn tick(&mut self) -> Result<bool> {
        let mut ctx = Context {
            devices: &mut self.devices,
            signaler: &mut self.process_controller,
            locals: &mut self.locals,
            globals: &mut self.globals,
        };

        ctx.devices.clear_stdout();

        // tick every running process
        for process in &mut self.processes {
            if process.status() == Status::Terminated {
                continue;
            }

            process.tick(&mut ctx)?;
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

        // free memory slots of terminated processes
        self.processes
            .iter()
            .filter(|process| process.status() == Status::Terminated)
            .for_each(|process| self.locals.free(process.id()));

        // remove terminated processes
        self.processes
            .retain(|process| process.status() != Status::Terminated);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::{Number, Opcode, ProcessDefinition, ProcessType};
    use crate::cart::Rom;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

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
            ProcessDefinition::new(ProcessType(1), vec![], vec![]),
        )]);

        Rom::new(with_main.into_iter().collect())
    }

    fn any_local_memory() -> impl LocalMemory {
        let mut memory = MockLocalMemory::new();
        memory.expect_poke().returning(|__, _, _| Ok(()));
        memory
            .expect_peek()
            .returning(|__, _| Ok(Value::Numeric(Number::Int(0))));
        memory.expect_free().returning(|__| ());
        memory
    }

    fn tracking_local_memory() -> (MockLocalMemory, Arc<Mutex<Vec<ProcessId>>>) {
        let freed = Arc::new(Mutex::new(Vec::new()));
        let mut memory = MockLocalMemory::new();
        memory.expect_poke().returning(|_, _, _| Ok(()));
        memory
            .expect_peek()
            .returning(|_, _| Ok(Value::Numeric(Number::Int(0))));
        let sink = freed.clone();
        memory
            .expect_free()
            .returning(move |id| sink.lock().unwrap().push(id));
        (memory, freed)
    }

    fn any_global_memory() -> impl GlobalMemory {
        let mut memory = MockGlobalMemory::new();
        memory.expect_poke().returning(|_, _| Ok(()));
        memory
            .expect_peek()
            .returning(|_| Ok(Value::Numeric(Number::Int(0))));
        memory
    }

    fn any_machine_with_rom(
        r: impl RomProvider,
    ) -> Machine<impl Devices, impl RomProvider, impl LocalMemory, impl GlobalMemory> {
        Machine::new(any_devices(), r, any_local_memory(), any_global_memory())
    }

    fn any_machine_with_rom_and_memory(
        r: impl RomProvider,
        locals: impl LocalMemory,
        globals: impl GlobalMemory,
    ) -> Machine<impl Devices, impl RomProvider, impl LocalMemory, impl GlobalMemory> {
        Machine::new(any_devices(), r, locals, globals)
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
                    vec![],
                ),
            ),
            (
                ProcessType(2),
                ProcessDefinition::new(ProcessType(2), vec![], vec![]),
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
                    vec![],
                ),
            ),
            (
                ProcessType(2),
                ProcessDefinition::new(
                    ProcessType(2),
                    vec![probe(2), yield_(), probe(2), yield_()],
                    vec![],
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

        let mut machine = Machine::new(devices, rom, any_local_memory(), any_global_memory());

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
                    vec![],
                ),
            ),
            (
                ProcessType(2),
                ProcessDefinition::new(
                    ProcessType(2),
                    vec![Instruction::new(Opcode::Yield, Location::default())],
                    vec![],
                ),
            ),
        ]));

        // Frame 1: process #1 spawns #2, then yields
        let mut machine = Machine::new(any_devices(), rom, any_local_memory(), any_global_memory());
        machine.tick().unwrap();
        assert_eq!(machine.processes.len(), 2);

        // Frame 2: process #1 kills #2, then terminates. Process #2 yields.
        machine.tick().unwrap();
        assert!(machine.processes.is_empty()); // no process remains
    }

    #[test]
    fn test_terminated_processes_free_memory_slots() {
        let (memory, freed) = tracking_local_memory();
        let mut machine =
            any_machine_with_rom_and_memory(any_rom(vec![]), memory, any_global_memory());
        machine.tick().unwrap(); // empty main process terminates immediately
        assert!(freed.lock().unwrap().contains(&ProcessId(1)));
    }
}
