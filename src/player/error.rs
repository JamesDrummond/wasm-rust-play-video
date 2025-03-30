use wasm_bindgen::prelude::*;
use web_sys::Element;
use crate::logger::Logger;
use crate::player::video::{get_element_by_id, VideoError};

#[wasm_bindgen]
pub fn show_error(message: &str) -> Result<(), JsValue> {
    Logger::info("Entering show_error()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    Logger::error(message).map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let error_element = get_element_by_id("errorMessage")?;
    error_element.set_text_content(Some(message));
    error_element.set_attribute("style", "display: block").map_err(|e| {
        let error = VideoError::VideoOperationFailed(format!("Failed to show error: {:?}", e));
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    Ok(())
}

#[wasm_bindgen]
pub fn hide_error() -> Result<(), JsValue> {
    Logger::info("Entering hide_error()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let error_element = get_element_by_id("errorMessage")?;
    error_element.set_attribute("style", "display: none").map_err(|e| {
        let error = VideoError::VideoOperationFailed(format!("Failed to hide error: {:?}", e));
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    Ok(())
} 