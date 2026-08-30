use crate::asm::Location;
use crate::devices::DeviceType;
use crate::error::{Result, RuntimeError};
use crate::vm::ValueStack;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConsoleApi {
    Log = 0x00,
}

impl TryFrom<u8> for ConsoleApi {
    type Error = RuntimeError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x00 => Ok(Self::Log),
            _ => Err(RuntimeError::InvalidApiCode(
                Location::default(),
                value,
                DeviceType::Console,
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleDevice {
    messages: Vec<String>,
}

impl ConsoleDevice {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn call_api(
        &mut self,
        raw_op: u8,
        stack: &mut impl ValueStack,
        location: Location,
    ) -> Result<bool> {
        match ConsoleApi::try_from(raw_op)? {
            ConsoleApi::Log => {
                let value = stack.pop(location)?;
                self.log(format!("{}", value));
                Ok(false)
            }
        }
    }

    pub fn messages(&self) -> &[String] {
        &self.messages
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn log(&mut self, message: String) {
        self.messages.push(message);
    }
}

impl Default for ConsoleDevice {
    fn default() -> Self {
        Self::new()
    }
}
