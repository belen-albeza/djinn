use crate::vm::{Devices, Result, Stacked};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DeviceType {
    #[default]
    Console = 0x00,
    Video = 0x01,
}

impl From<u8> for DeviceType {
    // NOTE: we can panic here because we are controlling the symbol values
    //       in the compiler itself.
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Console,
            0x01 => Self::Video,
            _ => unreachable!("Invalid device type: {}", value),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConsoleApi {
    Log = 0x00,
}

impl From<u8> for ConsoleApi {
    // NOTE: we can panic here because we are controlling the symbol values
    //       in the compiler itself.
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Log,
            _ => unreachable!("Invalid console API: {}", value),
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

    fn call_api(&mut self, raw_op: u8, cpu: &mut impl Stacked) -> Result<bool> {
        match ConsoleApi::from(raw_op) {
            ConsoleApi::Log => {
                let value = cpu.pop_stack()?;
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
    fn call_api<S: Stacked>(
        &mut self,
        device_type: DeviceType,
        api_op: u8,
        cpu: &mut S,
    ) -> Result<bool> {
        match device_type {
            DeviceType::Console => self.console.call_api(api_op, cpu),
            DeviceType::Video => unimplemented!(),
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
