use wasm_bindgen::prelude::*;

mod rest;
mod player;
mod logger;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub async fn init() -> Result<(), JsValue> {
    Ok(())
}
