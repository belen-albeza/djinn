use crate::asm::{Instruction, Location, Number, Opcode, Value};
use crate::vm::{Devices, ValueStack};
use crate::vm::{ProcessSignaler, Result};

pub(crate) mod stack;
use stack::Stack;
mod opcodes_alu;

#[derive(Debug)]
pub struct Context<'a, D: Devices, S: ProcessSignaler> {
    pub(crate) devices: &'a mut D,
    pub(crate) signaler: &'a mut S,
}

pub struct Cpu {
    pc: usize,
    stack: Stack,
    current_location: Location,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0,
            stack: Stack::default(),
            current_location: Location::default(),
        }
    }

    /// Executes an opcode and returns whether the process has yielded.
    pub fn exec_opcode<'a, D: Devices, S: ProcessSignaler>(
        &mut self,
        ctx: &mut Context<'a, D, S>,
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
                // FIXME: push process id as an actualy Value variant
                self.push_stack(Value::Numeric(Number::Int(process_id.0 as i32)));
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
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::{Location, Number, ProcessId, ProcessType, Value};
    use crate::devices::{ConsoleApi, DeviceType};
    use crate::vm::{MockDevices, MockProcessSignaler};

    fn any_cpu() -> Cpu {
        Cpu::default()
    }

    struct TestEnv {
        devices: MockDevices,
        signaler: MockProcessSignaler,
    }

    impl TestEnv {
        fn context(&mut self) -> Context<'_, MockDevices, MockProcessSignaler> {
            Context {
                devices: &mut self.devices,
                signaler: &mut self.signaler,
            }
        }
    }

    fn any_env() -> TestEnv {
        TestEnv {
            devices: any_devices(),
            signaler: any_signaler(),
        }
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
        let any_process_type = ProcessType(5);
        assert_eq!(
            cpu.exec_opcode(&mut env.context(), opcode(Opcode::Spawn(any_process_type))),
            Ok(false)
        );
        assert_eq!(cpu.pop_stack(), Ok(Value::Numeric(Number::Int(2))));
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
