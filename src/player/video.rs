use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;
use crate::logger::Logger;
use crate::player::error::{show_error, hide_error, VideoError};
use crate::player::state::VIDEO_STATE;
use crate::player::event_listeners::setup_event_listeners;

pub fn get_video_element() -> Result<HtmlVideoElement, VideoError> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    let video_element = document
        .get_element_by_id("videoPlayer")
        .ok_or_else(|| VideoError::ElementNotFound("videoPlayer".to_string()))?
        .dyn_into::<HtmlVideoElement>()
        .map_err(|_| VideoError::ElementNotFound("videoPlayer".to_string()))?;
    Ok(video_element)
}

pub fn get_element_by_id(id: &str) -> Result<web_sys::Element, VideoError> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    document
        .get_element_by_id(id)
        .ok_or_else(|| VideoError::ElementNotFound(id.to_string()))
}

#[wasm_bindgen]
pub fn get_wasm_initialized() -> bool {
    Logger::info("Entering get_wasm_initialized()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    }).unwrap_or_default();
    VIDEO_STATE.lock().unwrap().wasm_initialized
}

#[wasm_bindgen]
pub fn set_wasm_initialized(value: bool) -> Result<(), JsValue> {
    Logger::info("Entering set_wasm_initialized()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let mut result = VIDEO_STATE.lock().map_err(|e| {
        let error = VideoError::StateError(format!("Failed to lock state: {:?}", e));
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    if result.wasm_initialized != value {
        result.wasm_initialized = value;
    }
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn init_video() -> Result<(), JsValue> {
    setup_event_listeners()?;
    set_wasm_initialized(true)?;
    Ok(())
} 