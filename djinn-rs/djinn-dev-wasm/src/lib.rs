use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Emulator {
    cart: djinnc::Cartridge,
}

impl Emulator {
    pub fn new(cart: djinnc::Cartridge) -> Self {
        Self { cart }
    }
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.cart.title().to_string()
    }
}

#[wasm_bindgen]
pub fn build(title: &str) -> Result<Emulator, JsValue> {
    let cart = djinnc::build(title, "").map_err(|e| JsValue::from(e.to_string()))?;
    Ok(Emulator::new(cart))
}

#[wasm_bindgen(start)]
fn init() -> Result<(), JsValue> {
    Ok(())
}
