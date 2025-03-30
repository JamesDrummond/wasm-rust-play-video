use wasm_bindgen::prelude::*;
use crate::player::video::{get_video_element, get_element_by_id};
use crate::logger::Logger;
use crate::player::error::{VideoError, show_error};

#[wasm_bindgen]
pub fn get_video_time() -> Result<f64, JsValue> {
    Logger::info("Entering get_video_time()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let video_element = get_video_element()?;
    Ok(video_element.current_time())
}

#[wasm_bindgen]
pub fn get_video_duration() -> Result<f64, JsValue> {
    Logger::info("Entering get_video_duration()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let video_element = get_video_element()?;
    Ok(video_element.duration())
}

#[wasm_bindgen]
pub fn format_time(seconds: f64) -> String {
    let minutes = (seconds / 60.0).floor() as i32;
    let remaining_seconds = (seconds % 60.0).floor() as i32;
    format!("{}:{:02}", minutes, remaining_seconds)
}

#[wasm_bindgen]
pub async fn update_time_display() -> Result<(), JsValue> {
    let video_element = get_video_element()?;
    let current_time = video_element.current_time();
    let duration = video_element.duration();
    
    let current_time_display = get_element_by_id("currentTime")?;
    let total_time_display = get_element_by_id("totalTime")?;
    
    current_time_display.set_text_content(Some(&format_time(current_time)));
    if !duration.is_nan() {
        total_time_display.set_text_content(Some(&format_time(duration)));
    }
    
    Ok(())
} 