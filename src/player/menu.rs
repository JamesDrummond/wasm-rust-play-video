use wasm_bindgen::prelude::*;
use crate::logger::Logger;
use crate::player::error::{show_error, VideoError};
use crate::player::video::get_element_by_id;

#[wasm_bindgen]
pub fn position_playback_speed_menu(event_x: f64, event_y: f64) -> Result<(), JsValue> {
    Logger::info("Entering position_playback_speed_menu()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let window = web_sys::window().ok_or_else(|| {
        let error = VideoError::WindowNotFound;
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let _document = window.document().ok_or_else(|| {
        let error = VideoError::DocumentNotFound;
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    
    let playback_speed_menu = get_element_by_id("playbackSpeedMenu")?;
    
    // Position the menu at the event coordinates with a small offset
    playback_speed_menu.set_attribute("style", &format!("top: {}px; left: {}px", event_y, event_x + 5.0))
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to position menu: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    playback_speed_menu.set_attribute("class", "playback-speed-menu show")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to show menu: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    Ok(())
} 