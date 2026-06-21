use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

mod emulator;
mod error;
use emulator::Emulator;
use error::{DjinnError, DjinnErrorList};

#[derive(Deserialize, Tsify)]
#[tsify(from_wasm_abi)]
pub struct Project {
    title: String,
    #[serde(rename = "sourceCode")]
    source_code: String,
}

#[wasm_bindgen]
pub fn build(project: Project) -> Result<Emulator, DjinnErrorList> {
    let cart = djinnc::bundle(&project.title, &project.source_code.to_ascii_lowercase())
        .map_err(|e| DjinnErrorList(vec![DjinnError::from(e)]))?;

    Ok(Emulator::new(cart))
}

#[wasm_bindgen(start)]
fn init() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}
