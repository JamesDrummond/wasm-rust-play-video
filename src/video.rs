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

#[wasm_bindgen]
pub fn toggle_mute() -> Result<bool, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    video_element.set_muted(!video_element.muted());
    Ok(video_element.muted())
}

#[wasm_bindgen]
pub fn toggle_fullscreen() -> Result<bool, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    if document.fullscreen_element().is_some() {
        document.exit_fullscreen();
        Ok(false)
    } else {
        video_element.request_fullscreen()?;
        Ok(true)
    }
}

#[wasm_bindgen]
pub fn get_video_time() -> Result<f64, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    Ok(video_element.current_time())
}

#[wasm_bindgen]
pub fn get_video_duration() -> Result<f64, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    Ok(video_element.duration())
} 