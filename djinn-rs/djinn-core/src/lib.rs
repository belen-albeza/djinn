pub mod devices;
pub mod vm;

pub struct Cartridge {
    title: String,
}

impl Cartridge {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
