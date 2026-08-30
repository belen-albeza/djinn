use crate::asm::Location;
use crate::error::{Result, RuntimeError};
use crate::vm::{Devices, ValueStack};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DeviceType {
    #[default]
    Console = 0x00,
    Video = 0x01,
}

impl TryFrom<u8> for DeviceType {
    type Error = RuntimeError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x00 => Ok(Self::Console),
            0x01 => Ok(Self::Video),
            _ => Err(RuntimeError::InvalidDeviceType(Location::default(), value)),
        }
    }
}

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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VideoApi {
    Clear = 0x00,
}

impl TryFrom<u8> for VideoApi {
    type Error = RuntimeError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x00 => Ok(Self::Clear),
            _ => Err(RuntimeError::InvalidApiCode(
                Location::default(),
                value,
                DeviceType::Video,
            )),
        }
    }
}

pub const VIDEO_WIDTH: usize = 160;
pub const VIDEO_HEIGHT: usize = 144;

#[derive(Debug, Clone, Copy, PartialEq)]
struct VideoDevice {
    video_buffer: [u8; VIDEO_WIDTH * VIDEO_HEIGHT],
}

impl VideoDevice {
    fn new() -> Self {
        Self {
            video_buffer: [0; VIDEO_WIDTH * VIDEO_HEIGHT],
        }
    }

    fn buffer(&self) -> &[u8] {
        &self.video_buffer
    }

    fn clear_buffer(&mut self, color: u8) {
        self.video_buffer.fill(color.rem_euclid(16));
    }

    fn call_api(
        &mut self,
        raw_op: u8,
        stack: &mut impl ValueStack,
        location: Location,
    ) -> Result<bool> {
        match VideoApi::try_from(raw_op)? {
            VideoApi::Clear => {
                let color = stack.pop(location)?.as_int().rem_euclid(16);
                self.clear_buffer(color as u8);
                Ok(false)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleDevice {
    messages: Vec<String>,
}

impl ConsoleDevice {
    fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    fn call_api(
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

    fn messages(&self) -> &[String] {
        &self.messages
    }

    fn clear_messages(&mut self) {
        self.messages.clear();
    }

    fn log(&mut self, message: String) {
        self.messages.push(message);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeviceSet {
    video: VideoDevice,
    console: ConsoleDevice,
}

impl DeviceSet {
    pub fn new() -> Self {
        Self {
            video: VideoDevice::new(),
            console: ConsoleDevice::new(),
        }
    }
}

impl Default for DeviceSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Devices for DeviceSet {
    fn call_api<S: ValueStack>(
        &mut self,
        device_type: DeviceType,
        api_op: u8,
        stack: &mut S,
        location: Location,
    ) -> Result<bool> {
        match device_type {
            DeviceType::Console => self.console.call_api(api_op, stack, location),
            DeviceType::Video => self.video.call_api(api_op, stack, location),
        }
    }

    fn video_buffer(&self) -> &[u8] {
        self.video.buffer()
    }

    fn stdout(&self) -> &[String] {
        self.console.messages()
    }

    fn clear_stdout(&mut self) {
        self.console.clear_messages();
    }
}
