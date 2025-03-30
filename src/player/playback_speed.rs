use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;
use crate::logger::Logger;
use crate::player::error::{show_error, hide_error, VideoError};
use crate::player::state::VIDEO_STATE;
use crate::player::video::{get_video_element, get_element_by_id, hide_menus};

#[wasm_bindgen]
pub fn get_playback_speed() -> Result<f64, JsValue> {
    Logger::info("Entering get_playback_speed()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let video_element = get_video_element()?;
    Ok(video_element.playback_rate())
}

#[wasm_bindgen]
pub fn set_playback_speed(speed: f64) -> Result<(), JsValue> {
    Logger::info("Entering set_playback_speed()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let video_element = get_video_element()?;
    video_element.set_playback_rate(speed);
    let mut state = VIDEO_STATE.lock().map_err(|e| {
        let error = VideoError::StateError(format!("Failed to lock state: {:?}", e));
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    state.playback_speed = speed;
    update_playback_speed_active_state(speed)?;
    hide_menus()?;
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn get_current_playback_speed() -> f64 {
    Logger::info("Entering get_current_playback_speed()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    }).unwrap_or_default();
    VIDEO_STATE.lock().unwrap().playback_speed
}

#[wasm_bindgen]
pub fn update_playback_speed_active_state(speed: f64) -> Result<(), JsValue> {
    Logger::info("Entering update_playback_speed_active_state()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let playback_speed_menu = get_element_by_id("playbackSpeedMenu")?;
    let speed_options = playback_speed_menu.query_selector_all(".speed-option")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to get speed options: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    for i in 0..speed_options.length() {
        let option = speed_options.get(i)
            .ok_or_else(|| {
                let error = VideoError::VideoOperationFailed("Failed to get speed option".to_string());
                show_error(&error.to_string()).unwrap_or_default();
                error
            })?
            .dyn_into::<web_sys::Element>()
            .map_err(|e| {
                let error = VideoError::VideoOperationFailed(format!("Failed to convert Node to Element: {:?}", e));
                show_error(&error.to_string()).unwrap_or_default();
                error
            })?;
        
        let text_content = option.text_content()
            .ok_or_else(|| {
                let error = VideoError::VideoOperationFailed("No text content found".to_string());
                show_error(&error.to_string()).unwrap_or_default();
                error
            })?;
        
        let option_speed = text_content.replace('x', "").parse::<f64>()
            .map_err(|e| {
                let error = VideoError::VideoOperationFailed(format!("Failed to parse speed: {:?}", e));
                show_error(&error.to_string()).unwrap_or_default();
                error
            })?;
        
        if option_speed == speed {
            option.set_attribute("class", "speed-option active")
                .map_err(|e| {
                    let error = VideoError::VideoOperationFailed(format!("Failed to set active class: {:?}", e));
                    show_error(&error.to_string()).unwrap_or_default();
                    error
                })?;
        } else {
            option.set_attribute("class", "speed-option")
                .map_err(|e| {
                    let error = VideoError::VideoOperationFailed(format!("Failed to remove active class: {:?}", e));
                    show_error(&error.to_string()).unwrap_or_default();
                    error
                })?;
        }
    }
    
    hide_error()?;
    Ok(())
} 