use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;
use crate::player::error::VideoError;

#[wasm_bindgen]
pub fn toggle_picture_in_picture() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    
    let video_player = document
        .get_element_by_id("videoPlayer")
        .ok_or(VideoError::ElementNotFound("videoPlayer".to_string()))?
        .dyn_into::<HtmlVideoElement>()?;

    // Use JavaScript interop for Picture-in-Picture API
    let js_value = js_sys::Reflect::get(
        &window,
        &JsValue::from_str("document"),
    )?;
    let document = js_value.dyn_into::<web_sys::Document>()?;
    
    let pip_element = js_sys::Reflect::get(
        &document,
        &JsValue::from_str("pictureInPictureElement"),
    )?;

    if !pip_element.is_null() {
        let exit_pip = js_sys::Reflect::get(
            &document,
            &JsValue::from_str("exitPictureInPicture"),
        )?;
        let exit_pip_fn = exit_pip.dyn_into::<js_sys::Function>()?;
        // Call the function with document as this context
        exit_pip_fn.call1(&document, &JsValue::null())?;
    } else {
        let pip_enabled = js_sys::Reflect::get(
            &document,
            &JsValue::from_str("pictureInPictureEnabled"),
        )?;
        if pip_enabled.as_bool().unwrap_or(false) {
            // Get the requestPictureInPicture function
            let request_pip = js_sys::Reflect::get(
                &video_player,
                &JsValue::from_str("requestPictureInPicture"),
            )?;
            let request_pip_fn = request_pip.dyn_into::<js_sys::Function>()?;
            // Call the function with video_player as this context
            request_pip_fn.call1(&video_player, &JsValue::null())?;
        }
    }
    Ok(())
} 