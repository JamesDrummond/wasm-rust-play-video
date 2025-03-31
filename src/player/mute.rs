use wasm_bindgen::prelude::*;
use crate::logger::Logger;
use crate::player::{get_video_element, get_element_by_id};
use crate::player::error::{show_error, hide_error, VideoError};
use crate::player::state::VIDEO_STATE;

#[wasm_bindgen]
pub fn toggle_mute() -> Result<bool, JsValue> {
    Logger::info("Entering toggle_mute()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let video_element = get_video_element()?;
    let muted = !video_element.muted();
    
    Logger::info(&format!("Toggling mute state to: {}", muted))
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(e.to_string());
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    video_element.set_muted(muted);
    set_mute_button_text(muted)?;
    
    hide_error()?;
    Ok(video_element.muted())
}

#[wasm_bindgen]
pub fn is_video_muted() -> Result<bool, JsValue> {
    Logger::info("Entering is_video_muted()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let video_element = get_video_element()?;
    Ok(video_element.muted())
}

#[wasm_bindgen]
pub fn set_mute_button_text(is_muted: bool) -> Result<(), JsValue> {
    Logger::info("Entering set_mute_button_text()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let button = get_element_by_id("muteButton")?;
    button.set_text_content(Some(if is_muted { "Unmute" } else { "Mute" }));
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn update_mute_button_text() -> Result<(), JsValue> {
    Logger::info("Entering update_mute_button_text()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let is_muted = is_video_muted()?;
    let button = get_element_by_id("muteButton")?;
    button.set_text_content(Some(if is_muted { "Unmute" } else { "Mute" }));
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn get_is_muted() -> bool {
    Logger::info("Entering get_is_muted()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    }).unwrap_or_default();
    VIDEO_STATE.lock().unwrap().is_muted
}

#[wasm_bindgen]
pub fn set_is_muted(value: bool) -> Result<(), JsValue> {
    Logger::info("Entering set_is_muted()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let mut result = VIDEO_STATE.lock().map_err(|e| {
        let error = VideoError::StateError(format!("Failed to lock state: {:?}", e));
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    if result.is_muted != value {
        result.is_muted = value;
    }
    hide_error()?;
    Ok(())
} 