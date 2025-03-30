use wasm_bindgen::prelude::*;
use crate::logger::Logger;
use crate::player::error::{show_error, hide_error, VideoError};
use crate::player::video::get_element_by_id;

#[wasm_bindgen]
pub fn position_context_menu(event_x: f64, event_y: f64) -> Result<(), JsValue> {
    Logger::info("Entering position_context_menu()").map_err(|e| {
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
    
    let context_menu = get_element_by_id("contextMenu")?;
    
    // Position the menu at the event coordinates
    context_menu.set_attribute("style", &format!(
        "top: {}px; left: {}px",
        event_y + window.scroll_y().unwrap_or(0.0),
        event_x
    )).map_err(|e| {
        let error = VideoError::VideoOperationFailed(format!("Failed to position menu: {:?}", e));
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    
    // Toggle show class and hide playback speed menu
    context_menu.set_attribute("class", "context-menu show")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to show context menu: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    let playback_speed_menu = get_element_by_id("playbackSpeedMenu")?;
    playback_speed_menu.set_attribute("class", "playback-speed-menu")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to hide playback speed menu: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    hide_error()?;
    Ok(())
}

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

#[wasm_bindgen]
pub fn toggle_context_menu(x: f64, y: f64) -> Result<(), JsValue> {
    Logger::info("Entering toggle_context_menu()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    
    let context_menu = get_element_by_id("contextMenu")?;
    
    // Position the menu at the event coordinates
    context_menu.set_attribute("style", &format!("left: {}px; top: {}px;", x, y))
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to position menu: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Toggle show class and hide playback speed menu
    context_menu.set_attribute("class", "context-menu show")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to show context menu: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    let playback_speed_menu = get_element_by_id("playbackSpeedMenu")?;
    playback_speed_menu.set_attribute("class", "playback-speed-menu")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to hide playback speed menu: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn toggle_playback_speed_menu(x: f64, y: f64) -> Result<(), JsValue> {
    Logger::info("Entering toggle_playback_speed_menu()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    
    let playback_speed_menu = get_element_by_id("playbackSpeedMenu")?;
    
    // Position the menu at the event coordinates
    playback_speed_menu.set_attribute("style", &format!("left: {}px; top: {}px;", x, y))
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
    
    hide_error()?;
    Ok(())
} 