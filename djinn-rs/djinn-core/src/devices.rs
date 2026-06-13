use crate::vm::Devices;

const VIDEO_WIDTH: usize = 160;
const VIDEO_HEIGHT: usize = 144;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeviceSet {
    video: VideoDevice,
}

impl DeviceSet {
    pub fn new() -> Self {
        Self {
            video: VideoDevice::new(),
        }
    }
}

impl Default for DeviceSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Devices for DeviceSet {
    fn video_buffer(&self) -> &[u8] {
        self.video.buffer()
    }
}
