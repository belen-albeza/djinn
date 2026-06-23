use crate::asm::{Instruction, Location, Opcode, ProcessId, Value};
use crate::error::RuntimeError;
use crate::vm::{Devices, Memory, ValueStack};
use crate::vm::{ProcessSignaler, Result};

pub(crate) mod stack;
use stack::Stack;
mod opcodes_alu;

#[derive(Debug)]
pub struct Context<'a, D: Devices, S: ProcessSignaler, M: Memory> {
    pub(crate) devices: &'a mut D,
    pub(crate) signaler: &'a mut S,
    pub(crate) locals: &'a mut M,
}

pub struct Cpu {
    pc: usize,
    stack: Stack,
    current_location: Location,
    id: ProcessId,
}

impl Cpu {
    pub fn new(id: ProcessId) -> Self {
        Self {
            pc: 0,
            id,
            stack: Stack::default(),
            current_location: Location::default(),
        }
    }

    /// Executes an opcode and returns whether the process has yielded.
    pub fn exec_opcode<'a, D: Devices, S: ProcessSignaler, M: Memory>(
        &mut self,
        ctx: &mut Context<'a, D, S, M>,
        instruction: Instruction,
    ) -> Result<bool> {
        let Instruction { opcode, location } = instruction;
        self.current_location = location;

        match opcode {
            Opcode::NoOp => Ok(false),
            Opcode::Device(device_type, api_op) => {
                ctx.devices
                    .call_api(device_type, api_op, &mut self.stack, self.current_location)
            }
            Opcode::Yield => Ok(true),
            Opcode::Spawn(process_type) => {
                let process_id = ctx.signaler.spawn(process_type);
                self.push_stack(Value::Process(process_id));
                Ok(false)
            }
            Opcode::Kill => {
                let process_id = self.pop_stack()?.try_into()?;
                ctx.signaler.kill(process_id);
                Ok(false)
            }
            Opcode::Push(value) => {
                self.push_stack(value);
                Ok(false)
            }
            Opcode::Pop => {
                self.pop_stack()?;
                Ok(false)
            }
            Opcode::Dup => {
                let value = self.pop_stack()?;
                self.push_stack(value);
                self.push_stack(value);
                Ok(false)
            }
            Opcode::Stl(addr) => {
                let value = self.pop_stack()?;
                ctx.locals
                    .poke(self.id, addr, value)
                    .map_err(|e: RuntimeError| e.with_location(self.current_location))?;
                Ok(false)
            }
            Opcode::Ldl(addr) => {
                let value = ctx
                    .locals
                    .peek(self.id, addr)
                    .map_err(|e: RuntimeError| e.with_location(self.current_location))?;
                self.push_stack(value);
                Ok(false)
            }
            Opcode::Not => self.exec_opcode_not(),
            Opcode::And => self.exec_opcode_and(),
            Opcode::Or => self.exec_opcode_or(),
            Opcode::Xor => self.exec_opcode_xor(),
            Opcode::Add => self.exec_opcode_add(),
            Opcode::Sub => self.exec_opcode_sub(),
            Opcode::Mul => self.exec_opcode_mul(),
            Opcode::Div => self.exec_opcode_div(),
            Opcode::Mod => self.exec_opcode_rem(),
            Opcode::Eq => self.exec_opcode_eq(),
            Opcode::Neq => self.exec_opcode_neq(),
            Opcode::Lt => self.exec_opcode_lt(),
            Opcode::Leq => self.exec_opcode_leq(),
            Opcode::Gt => self.exec_opcode_gt(),
            Opcode::Geq => self.exec_opcode_geq(),
            Opcode::Inc => self.exec_opcode_inc(),
            Opcode::Dec => self.exec_opcode_dec(),
        }
    }

    /// Reads the next opcode from the instruction slice.
    pub fn read_opcode(&mut self, instructions: &[Instruction]) -> Option<Instruction> {
        let instruction = instructions.get(self.pc)?;
        self.pc += 1;
        Some(*instruction)
    }

    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop_stack(&mut self) -> Result<Value> {
        self.stack.pop(self.current_location)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new(ProcessId::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::{Location, Number, ProcessId, ProcessType, Value};
    use crate::devices::{ConsoleApi, DeviceType};
    use crate::vm::{MockDevices, MockMemory, MockProcessSignaler};

    use mockall::predicate::*;

    fn any_cpu() -> Cpu {
        Cpu::default()
    }

    struct TestEnv {
        devices: MockDevices,
        signaler: MockProcessSignaler,
        locals: MockMemory,
    }

    impl TestEnv {
        fn context(&mut self) -> Context<'_, MockDevices, MockProcessSignaler, MockMemory> {
            Context {
                devices: &mut self.devices,
                signaler: &mut self.signaler,
                locals: &mut self.locals,
            }
        }
    }

    fn any_env() -> TestEnv {
        TestEnv {
            devices: any_devices(),
            signaler: any_signaler(),
            locals: any_memory(),
        }
    }

    fn any_memory() -> MockMemory {
        let mut memory = MockMemory::new();
        memory.expect_poke().returning(|__, _, _| Ok(()));
        memory
            .expect_peek()
            .returning(|__, _| Ok(Value::Numeric(Number::Int(0))));
        memory
    }

    fn any_devices() -> MockDevices {
        let mut devices = MockDevices::new();
        devices.expect_call_api().returning(|_, _, _, _| Ok(false));
        devices.expect_video_buffer().return_const(Vec::<u8>::new());
        devices.expect_stdout().return_const(vec![]);

        devices
    }

    fn any_signaler() -> MockProcessSignaler {
        let mut signaler = MockProcessSignaler::new();
        signaler.expect_spawn().returning(|_| ProcessId(2));
        signaler
    }

    fn opcode(opcode: Opcode) -> Instruction {
        Instruction::new(opcode, Location::default())
    }

    #[test]
    fn test_yield_opcode() {
        let mut cpu = any_cpu();
        let mut env = any_env();
        assert_eq!(
            cpu.exec_opcode(&mut env.context(), opcode(Opcode::Yield)),
            Ok(true)
        );
    }

    #[test]
    fn test_noop_opcode() {
        let mut cpu = any_cpu();
        let mut env = any_env();
        assert_eq!(
            cpu.exec_opcode(&mut env.context(), opcode(Opcode::NoOp)),
            Ok(false)
        );
    }

    #[test]
    fn test_spawn_opcode() {
        let mut cpu = any_cpu();
        let mut env = any_env();
        let mut signaler = MockProcessSignaler::new();
        signaler.expect_spawn().times(1).returning(|_| ProcessId(2));
        env.signaler = signaler;

        let any_process_type = ProcessType(5);
        assert_eq!(
            cpu.exec_opcode(&mut env.context(), opcode(Opcode::Spawn(any_process_type))),
            Ok(false)
        );
        assert_eq!(cpu.pop_stack(), Ok(Value::Process(ProcessId(2))));
    }

    #[test]
    fn test_kill_opcode() {
        let mut cpu = any_cpu();
        let mut env = any_env();

        let mut signaler = MockProcessSignaler::new();
        signaler
            .expect_kill()
            .withf(|id| *id == ProcessId(2))
            .times(1)
            .returning(|_| ());
        env.signaler = signaler;

        cpu.stack.push(Value::Process(ProcessId(2)));
        assert_eq!(
            cpu.exec_opcode(&mut env.context(), opcode(Opcode::Kill)),
            Ok(false)
        );
        assert!(cpu.stack.is_empty());
    }

    #[test]
    fn test_push_opcode() {
        let mut cpu = any_cpu();
        let mut env = any_env();
        assert_eq!(
            cpu.exec_opcode(
                &mut env.context(),
                opcode(Opcode::Push(Value::Numeric(Number::Int(1))))
            ),
            Ok(false)
        );
        assert_eq!(
            cpu.stack.pop(Location::default()),
            Ok(Value::Numeric(Number::Int(1)))
        );
    }

    #[test]
    fn test_pop_opcode() {
        let mut cpu = any_cpu();
        let mut env = any_env();
        cpu.stack.push(Value::Numeric(Number::Int(1)));

        assert_eq!(
            cpu.exec_opcode(&mut env.context(), opcode(Opcode::Pop)),
            Ok(false)
        );
        assert!(cpu.stack.is_empty());
    }

    #[test]
    fn test_dup_opcode() {
        let mut cpu = any_cpu();
        let mut env = any_env();
        cpu.stack.push(Value::Numeric(Number::Int(1)));

        assert_eq!(
            cpu.exec_opcode(&mut env.context(), opcode(Opcode::Dup)),
            Ok(false)
        );
        assert_eq!(
            cpu.stack.pop(Location::default()),
            Ok(Value::Numeric(Number::Int(1)))
        );
        assert_eq!(
            cpu.stack.pop(Location::default()),
            Ok(Value::Numeric(Number::Int(1)))
        );
    }

    #[test]
    fn test_ldl_opcode() {
        let mut cpu = any_cpu();
        let mut env = any_env();
        let mut memory = MockMemory::new();
        memory
            .expect_peek()
            .with(eq(cpu.id), eq(3))
            .times(1)
            .returning(|_, _| Ok(Value::Numeric(Number::Int(42))));
        env.locals = memory;

        assert_eq!(
            cpu.exec_opcode(&mut env.context(), opcode(Opcode::Ldl(3))),
            Ok(false)
        );
        assert_eq!(
            cpu.stack.pop(Location::default()),
            Ok(Value::Numeric(Number::Int(42)))
        );
    }

    #[test]
    fn test_ldl_with_invalid_address() {
        let mut cpu = any_cpu();
        let mut env = any_env();
        let mut memory = MockMemory::new();
        memory.expect_peek().returning(|_, _| {
            Err(RuntimeError::LocalNotFound(
                Location::default(),
                ProcessId(1),
                3,
            ))
        });
        env.locals = memory;

        assert_eq!(
            cpu.exec_opcode(
                &mut env.context(),
                Instruction::new(Opcode::Ldl(3), Location { line: 2, column: 3 })
            ),
            Err(RuntimeError::LocalNotFound(
                Location { line: 2, column: 3 },
                ProcessId(1),
                3
            ))
        );
    }

    #[test]
    fn test_stl_opcode() {
        let mut cpu = any_cpu();
        let mut env = any_env();
        let mut memory = MockMemory::new();
        memory
            .expect_poke()
            .with(eq(cpu.id), eq(3), eq(Value::Numeric(Number::Int(42))))
            .times(1)
            .returning(|_, _, _| Ok(()));
        env.locals = memory;

        cpu.push_stack(Value::Numeric(Number::Int(42)));

        assert_eq!(
            cpu.exec_opcode(&mut env.context(), opcode(Opcode::Stl(3))),
            Ok(false)
        );
        assert!(cpu.stack.is_empty());
    }

    #[test]
    fn test_device_opcode() {
        let mut cpu = any_cpu();
        let mut env = any_env();
        let mut devices = MockDevices::new();
        devices
            .expect_call_api()
            .withf(|device_type, api_op, _, _| {
                *device_type == DeviceType::Console && *api_op == ConsoleApi::Log as u8
            })
            .times(1)
            .returning(|_, _, _, _| Ok(false));
        env.devices = devices;

        let res = cpu.exec_opcode(
            &mut env.context(),
            opcode(Opcode::Device(DeviceType::Console, ConsoleApi::Log as u8)),
        );
        assert_eq!(res, Ok(false));
    }
}
