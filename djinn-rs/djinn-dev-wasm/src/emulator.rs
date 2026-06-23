use wasm_bindgen::prelude::*;

use djinn_core::cart::Rom;
use djinn_core::devices::{DeviceSet, VIDEO_HEIGHT, VIDEO_WIDTH};
use djinn_core::vm::{Devices, Machine, memory::Locals};

use crate::error::DjinnError;

const DISPLAY_BYTES_PER_PIXEL: usize = 4;
const DISPLAY_LEN: usize = VIDEO_WIDTH * VIDEO_HEIGHT * DISPLAY_BYTES_PER_PIXEL;

static mut DISPLAY_BUFFER: [u8; DISPLAY_LEN] = [0; DISPLAY_LEN];

const THEME: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // 0
    (0x1D, 0x2B, 0x53), // 1
    (0x7E, 0x25, 0x53), // 2
    (0x00, 0x87, 0x51), // 3
    (0xAB, 0x52, 0x36), // 4
    (0x5F, 0x57, 0x4F), // 5
    (0xC2, 0xC3, 0xC7), // 6
    (0xFF, 0xF1, 0xE8), // 7
    (0xFF, 0x00, 0x4D), // 8
    (0xFF, 0xA3, 0x00), // 9
    (0xFF, 0xEC, 0x27), // a
    (0x00, 0xE4, 0x36), // b
    (0x29, 0xAD, 0xFF), // c
    (0x83, 0x76, 0x9C), // d
    (0xFF, 0x77, 0xA8), // e
    (0xFF, 0xCC, 0xAA), // f
];

#[wasm_bindgen]
pub struct Emulator {
    title: String,
    vm: Machine<DeviceSet, Rom, Locals>,
}

impl Emulator {
    pub fn new(cart: djinnc::Cartridge) -> Self {
        Self {
            title: cart.title().to_string(),
            vm: Machine::new(DeviceSet::default(), cart.rom().clone(), Locals::new()),
        }
    }

    fn update_display_buffer(&self) {
        let video_buffer = self.vm.devices().video_buffer();
        for (i, pixel) in video_buffer.iter().enumerate() {
            let (r, g, b) = THEME[*pixel as usize];
            let offset = i * 4;
            unsafe {
                DISPLAY_BUFFER[offset] = r;
                DISPLAY_BUFFER[offset + 1] = g;
                DISPLAY_BUFFER[offset + 2] = b;
                DISPLAY_BUFFER[offset + 3] = 0xff;
            }
        }
    }
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen]
    pub fn tick(&mut self) -> Result<bool, DjinnError> {
        let shall_halt = self
            .vm
            .tick()
            .map_err(|e| DjinnError::new(e.location().into(), e.to_string()))?;
        self.update_display_buffer();

        Ok(shall_halt)
    }

    #[wasm_bindgen(getter)]
    pub fn memory() -> JsValue {
        wasm_bindgen::memory()
    }

    // NOTE: WebAssembly is a single-threaded environment, so a static mutable
    // reference is OK.
    #[wasm_bindgen(getter, js_name = "displayBuffer")]
    #[allow(static_mut_refs)]
    pub fn display_buffer(&self) -> *const u8 {
        unsafe { DISPLAY_BUFFER.as_ptr() }
    }

    #[wasm_bindgen(getter)]
    pub fn stdout(&self) -> Vec<String> {
        self.vm.devices().stdout().to_owned()
    }
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.to_owned()
    }
}
