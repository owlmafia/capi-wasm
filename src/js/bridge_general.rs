use log::info;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=initLog)]
pub fn init_log() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Initialized wasm logs");
    Ok(())
}
