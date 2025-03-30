use wasm_bindgen::prelude::*;
use crate::logger::Logger;
use crate::player::video::{get_video_element, get_element_by_id};
use crate::player::error::{show_error, hide_error, VideoError};

#[wasm_bindgen]
pub fn toggle_fullscreen() -> Result<bool, JsValue> {
    Logger::info("Entering toggle_fullscreen()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let window = web_sys::window().ok_or_else(|| {
        let error = VideoError::WindowNotFound;
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let document = window.document().ok_or_else(|| {
        let error = VideoError::DocumentNotFound;
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let video_element = get_video_element()?;
    
    if document.fullscreen_element().is_some() {
        Logger::info("Exiting fullscreen mode")
            .map_err(|e| {
                let error = VideoError::VideoOperationFailed(e.to_string());
                show_error(&error.to_string()).unwrap_or_default();
                error
            })?;
        document.exit_fullscreen();
        hide_error()?;
        Ok(false)
    } else {
        Logger::info("Entering fullscreen mode")
            .map_err(|e| {
                let error = VideoError::VideoOperationFailed(e.to_string());
                show_error(&error.to_string()).unwrap_or_default();
                error
            })?;
        video_element.request_fullscreen().map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to enter fullscreen: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
        hide_error()?;
        Ok(true)
    }
}

#[wasm_bindgen]
pub fn is_fullscreen() -> Result<bool, JsValue> {
    Logger::info("Entering is_fullscreen()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let window = web_sys::window().ok_or_else(|| {
        let error = VideoError::WindowNotFound;
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let document = window.document().ok_or_else(|| {
        let error = VideoError::DocumentNotFound;
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    Ok(document.fullscreen_element().is_some())
}

#[wasm_bindgen]
pub fn set_fullscreen_button_text() -> Result<(), JsValue> {
    Logger::info("Entering set_fullscreen_button_text()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let is_fullscreen = toggle_fullscreen()?;
    let button = get_element_by_id("fullscreenButton")?;
    button.set_text_content(Some(if is_fullscreen { "Exit Fullscreen" } else { "Fullscreen" }));
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn update_fullscreen_button_text() -> Result<(), JsValue> {
    Logger::info("Entering update_fullscreen_button_text()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let is_fullscreen = is_fullscreen()?;
    let button = get_element_by_id("fullscreenButton")?;
    button.set_text_content(Some(if is_fullscreen { "Exit Fullscreen" } else { "Fullscreen" }));
    hide_error()?;
    Ok(())
} 