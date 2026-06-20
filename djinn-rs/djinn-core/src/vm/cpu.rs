use crate::asm::{Instruction, Location, Opcode, Value};
use crate::vm::{Devices, Stacked};
use crate::vm::{Result, RuntimeError};

mod stack;
use stack::Stack;
mod opcodes_alu;

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
    pub fn exec_opcode(
        &mut self,
        devices: &mut impl Devices,
        instruction: Instruction,
    ) -> Result<bool> {
        let Instruction { opcode, location } = instruction;
        self.current_location = location;

        match opcode {
            Opcode::NoOp => Ok(false),
            Opcode::Device(device_type, api_op) => devices.call_api(device_type, api_op, self),
            Opcode::Yield => Ok(true),
            Opcode::Push(value) => {
                self.stack.push(value);
                Ok(false)
            }
            Opcode::Pop => {
                self.pop_stack()?;
                Ok(false)
            }
            Opcode::Dup => {
                let value = self.pop_stack()?;
                self.stack.push(value);
                self.stack.push(value);
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
}

impl Stacked for Cpu {
    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop_stack(&mut self) -> Result<Value> {
        let value = self
            .stack
            .pop()
            .ok_or(RuntimeError::StackUnderflow(self.current_location))?;
        Ok(value)
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
    use crate::asm::{Location, Number, Value};
    use crate::devices::{ConsoleApi, DeviceType};
    use crate::vm::MockDevices;

    fn any_cpu() -> Cpu {
        Cpu::default()
    }

    fn any_devices() -> impl Devices {
        let mut devices = MockDevices::new();
        devices.expect_call_api().returning(|_, _, _| Ok(false));
        devices.expect_video_buffer().return_const(Vec::<u8>::new());
        devices.expect_stdout().returning(String::new);
        devices
    }

    fn opcode(opcode: Opcode) -> Instruction {
        Instruction::new(opcode, Location::default())
    }

    #[test]
    fn test_yield_opcode() {
        let mut cpu = any_cpu();
        assert_eq!(
            cpu.exec_opcode(&mut any_devices(), opcode(Opcode::Yield)),
            Ok(true)
        );
    }

    #[test]
    fn test_noop_opcode() {
        let mut cpu = any_cpu();
        assert_eq!(
            cpu.exec_opcode(&mut any_devices(), opcode(Opcode::NoOp)),
            Ok(false)
        );
    }

    #[test]
    fn test_push_opcode() {
        let mut cpu = any_cpu();
        assert_eq!(
            cpu.exec_opcode(
                &mut any_devices(),
                opcode(Opcode::Push(Value::Numeric(Number::Int(1))))
            ),
            Ok(false)
        );
        assert_eq!(cpu.pop_stack(), Ok(Value::Numeric(Number::Int(1))));
    }

    #[test]
    fn test_pop_opcode() {
        let mut cpu = any_cpu();
        cpu.stack.push(Value::Numeric(Number::Int(1)));

        assert_eq!(
            cpu.exec_opcode(&mut any_devices(), opcode(Opcode::Pop)),
            Ok(false)
        );
        assert!(cpu.stack.is_empty());
    }

    #[test]
    fn test_dup_opcode() {
        let mut cpu = any_cpu();
        cpu.stack.push(Value::Numeric(Number::Int(1)));

        assert_eq!(
            cpu.exec_opcode(&mut any_devices(), opcode(Opcode::Dup)),
            Ok(false)
        );
        assert_eq!(cpu.pop_stack(), Ok(Value::Numeric(Number::Int(1))));
        assert_eq!(cpu.pop_stack(), Ok(Value::Numeric(Number::Int(1))));
    }

    #[test]
    fn test_device_opcode() {
        let mut cpu = any_cpu();
        let mut devices = MockDevices::new();
        devices
            .expect_call_api()
            .withf(|device_type, api_op, _| {
                *device_type == DeviceType::Console && *api_op == ConsoleApi::Log as u8
            })
            .times(1)
            .returning(|_, _, _| Ok(false));
        let res = cpu.exec_opcode(
            &mut devices,
            opcode(Opcode::Device(DeviceType::Console, ConsoleApi::Log as u8)),
        );
        assert_eq!(res, Ok(false));
    }
}
