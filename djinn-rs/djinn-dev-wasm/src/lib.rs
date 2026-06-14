use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

mod emulator;
mod error;
use emulator::Emulator;
use error::DjinnErrorList;

#[derive(Deserialize, Tsify)]
#[tsify(from_wasm_abi)]
pub struct Project {
    title: String,
    #[serde(rename = "sourceCode")]
    source_code: String,
}

#[wasm_bindgen]
pub fn build(project: Project) -> Result<Emulator, DjinnErrorList> {
    // TODO: Convert from djinnc errors to BuildErrorList
    let cart =
        djinnc::build(&project.title, &project.source_code).expect("Failed to build cartridge");
    Ok(Emulator::new(cart))
    // let errors = BuildErrorList(vec![
    //     BuildError {
    //         position: (1, 1),
    //         message: "Unexpected character `*`".to_string(),
    //     },
    //     BuildError {
    //         position: (1, 1),
    //         message: "`main` process not found".to_string(),
    //     },
    // ]);
    // Err(errors)
}

#[wasm_bindgen(start)]
fn init() -> Result<(), JsValue> {
    Ok(())
}
