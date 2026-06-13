#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Invalid ROM")]
    LoadRomError,
}

type Result<T> = std::result::Result<T, RuntimeError>;

pub trait Devices {
    fn video_buffer(&self) -> &[u8];
}

pub struct Machine<D: Devices> {
    devices: D,
}

impl<D: Devices> Machine<D> {
    pub fn new(devices: D) -> Self {
        Self { devices }
    }

    pub fn step(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn devices(&self) -> &D {
        &self.devices
    }
}
