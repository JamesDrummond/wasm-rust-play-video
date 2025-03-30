use wasm_bindgen::prelude::*;
use crate::logger::Logger;
use crate::player::video::{get_video_element, get_element_by_id};
use crate::player::error::{show_error, hide_error, VideoError};

#[wasm_bindgen]
pub async fn set_toggle_play(text: &str) -> Result<(), JsValue> {
    Logger::info("Entering set_toggle_play()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let button = get_element_by_id("toggleButton")?;
    button.set_text_content(Some(text));
    Ok(())
}

#[wasm_bindgen]
pub async fn play_video() -> Result<(), JsValue> {
    Logger::info("Entering play_video()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let video_element = get_video_element()?;
    let is_video_playing = is_video_playing()?;
    
    if is_video_playing {
        Logger::info("Pausing video").map_err(|e| {
            let error = VideoError::VideoOperationFailed(e.to_string());
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
        let _ = video_element.pause().map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to pause video: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
        set_toggle_play("Play").await?;
    } else {
        Logger::info("Playing video").map_err(|e| {
            let error = VideoError::VideoOperationFailed(e.to_string());
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
        let _ = video_element.play().map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to play video: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
        set_toggle_play("Pause").await?;
    }
    
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn is_video_playing() -> Result<bool, JsValue> {
    Logger::info("Entering is_video_playing()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let video_element = get_video_element()?;
    Ok(!video_element.paused())
} 



