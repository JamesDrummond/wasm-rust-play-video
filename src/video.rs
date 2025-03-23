use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;

#[wasm_bindgen]
pub fn play_video() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    video_element.play()?;
    Ok(())
}

#[wasm_bindgen]
pub fn pause_video() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    video_element.pause()?;
    Ok(())
} 