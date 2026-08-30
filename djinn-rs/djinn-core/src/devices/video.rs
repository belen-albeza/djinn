use crate::asm::Location;
use crate::devices::DeviceType;
use crate::error::{Result, RuntimeError};
use crate::vm::ValueStack;

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
pub struct VideoDevice {
    video_buffer: [u8; VIDEO_WIDTH * VIDEO_HEIGHT],
}

impl VideoDevice {
    pub fn new() -> Self {
        Self {
            video_buffer: [0; VIDEO_WIDTH * VIDEO_HEIGHT],
        }
    }

    pub fn buffer(&self) -> &[u8] {
        &self.video_buffer
    }

    pub fn clear_buffer(&mut self, color: u8) {
        self.video_buffer.fill(color.rem_euclid(16));
    }

    pub fn call_api(
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

impl Default for VideoDevice {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_buffer() {
        let mut video_device = VideoDevice::new();
        video_device.clear_buffer(0x0f);
        assert_eq!(video_device.buffer(), &[0x0f; VIDEO_WIDTH * VIDEO_HEIGHT]);
    }
}
