use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, Window};

mod mock_client;
mod post_client;
mod video;
mod logger;
mod example;
use mock_client::MockHttpClient;
use post_client::fetch_post;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub async fn init() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Window not found"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("Document not found"))?;
    


    Ok(())
}
