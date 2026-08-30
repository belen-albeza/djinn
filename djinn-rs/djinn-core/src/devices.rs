use crate::asm::Location;
use crate::error::{Result, RuntimeError};
use crate::vm::{Devices, ValueStack};

pub mod console;
pub mod video;

pub use video::{VIDEO_HEIGHT, VIDEO_WIDTH};

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

#[derive(Debug, Clone, PartialEq)]
pub struct DeviceSet {
    video: video::VideoDevice,
    console: console::ConsoleDevice,
}

impl DeviceSet {
    pub fn new() -> Self {
        Self {
            video: video::VideoDevice::default(),
            console: console::ConsoleDevice::default(),
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
