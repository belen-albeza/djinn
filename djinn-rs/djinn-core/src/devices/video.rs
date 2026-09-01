use crate::asm::{Location, Number, Value};
use crate::devices::DeviceType;
use crate::error::{Result, RuntimeError};
use crate::vm::ValueStack;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VideoApi {
    Clear = 0x00,
    PutPixel = 0x01,
    GetPixel = 0x02,
}

impl TryFrom<u8> for VideoApi {
    type Error = RuntimeError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x00 => Ok(Self::Clear),
            0x01 => Ok(Self::PutPixel),
            0x02 => Ok(Self::GetPixel),
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
    z_buffer: [i32; VIDEO_WIDTH * VIDEO_HEIGHT],
}

impl VideoDevice {
    pub fn new() -> Self {
        Self {
            video_buffer: [0; VIDEO_WIDTH * VIDEO_HEIGHT],
            z_buffer: [i32::MIN; VIDEO_WIDTH * VIDEO_HEIGHT],
        }
    }

    pub fn buffer(&self) -> &[u8] {
        &self.video_buffer
    }

    pub fn clear_buffer(&mut self, color: u8) {
        self.video_buffer.fill(color.rem_euclid(16));
        self.z_buffer.fill(i32::MIN);
    }

    pub fn call_api(
        &mut self,
        raw_op: u8,
        stack: &mut impl ValueStack,
        location: Location,
    ) -> Result<bool> {
        match VideoApi::try_from(raw_op)? {
            VideoApi::Clear => self.exec_clear_buffer(stack, location),
            VideoApi::PutPixel => self.exec_put_pixel(stack, location),
            VideoApi::GetPixel => self.exec_get_pixel(stack, location),
        }
    }

    fn put_pixel(&mut self, x: i32, y: i32, z: i32, color: u8) {
        let Some(index) = Self::index(x, y) else {
            return;
        };

        if z >= self.z_buffer[index] {
            self.video_buffer[index] = color;
            self.z_buffer[index] = z;
        }
    }

    #[inline]
    fn index(x: i32, y: i32) -> Option<usize> {
        // out of bounds check
        if x < 0 || x as usize >= VIDEO_WIDTH || y < 0 || y as usize >= VIDEO_HEIGHT {
            None
        } else {
            Some(y as usize * VIDEO_WIDTH + x as usize)
        }
    }

    fn exec_clear_buffer(
        &mut self,
        stack: &mut impl ValueStack,
        location: Location,
    ) -> Result<bool> {
        let color = stack.pop(location)?.as_int().rem_euclid(16);
        self.clear_buffer(color as u8);
        Ok(false)
    }

    fn exec_put_pixel(&mut self, stack: &mut impl ValueStack, location: Location) -> Result<bool> {
        let color = stack.pop(location)?.as_int().rem_euclid(15) as u8;
        let z = stack.pop(location)?.as_int();
        let y = stack.pop(location)?.as_int();
        let x = stack.pop(location)?.as_int();

        self.put_pixel(x, y, z, color);
        Ok(false)
    }

    fn exec_get_pixel(&mut self, stack: &mut impl ValueStack, location: Location) -> Result<bool> {
        let y = stack.pop(location)?.as_int();
        let x = stack.pop(location)?.as_int();

        let Some(index) = Self::index(x, y) else {
            return Ok(false);
        };

        let color = self.video_buffer[index] as i32;

        stack.push(Value::Numeric(Number::Int(color)));
        Ok(false)
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
    use crate::vm::Stack;

    #[test]
    fn test_clear_buffer() {
        let mut video_device = VideoDevice::new();
        video_device.clear_buffer(0x0f);
        assert_eq!(video_device.buffer(), &[0x0f; VIDEO_WIDTH * VIDEO_HEIGHT]);
    }

    #[test]
    fn test_put_pixel() {
        let mut video_device = VideoDevice::new();
        video_device.put_pixel(0, 0, 0, 0x0f);
        assert_eq!(video_device.buffer()[0], 0x0f);
    }

    #[test]
    fn test_put_pixel_over_existing_pixel() {
        let mut video_device = VideoDevice::new();
        video_device.put_pixel(0, 0, 1, 0x0f);
        video_device.put_pixel(0, 0, 0, 0x0e);
        assert_eq!(video_device.buffer()[0], 0x0f);

        video_device.put_pixel(0, 0, 2, 0x0a);
        assert_eq!(video_device.buffer()[0], 0x0a);
    }

    #[test]
    fn test_exec_get_pixel() {
        let mut video_device = VideoDevice::new();
        video_device.video_buffer[32 + VIDEO_WIDTH * 32] = 0x0f;

        let mut stack = Stack::from_iter([
            Value::Numeric(Number::Int(32)),
            Value::Numeric(Number::Int(32)),
        ]);

        let result = video_device.exec_get_pixel(&mut stack, Location::default());
        assert!(result.is_ok());

        assert_eq!(stack.into_values(), vec![Value::Numeric(Number::Int(0x0f))]);
    }
}
