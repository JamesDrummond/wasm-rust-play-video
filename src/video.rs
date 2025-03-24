use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::logger::Logger;

#[derive(Debug)]
pub enum VideoError {
    WindowNotFound,
    DocumentNotFound,
    ElementNotFound(String),
    VideoOperationFailed(String),
    StateError(String),
}

impl std::fmt::Display for VideoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoError::WindowNotFound => write!(f, "Window not found"),
            VideoError::DocumentNotFound => write!(f, "Document not found"),
            VideoError::ElementNotFound(id) => write!(f, "Element not found: {}", id),
            VideoError::VideoOperationFailed(msg) => write!(f, "Video operation failed: {}", msg),
            VideoError::StateError(msg) => write!(f, "State error: {}", msg),
        }
    }
}

impl std::error::Error for VideoError {}

impl From<VideoError> for JsValue {
    fn from(error: VideoError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

static VIDEO_STATE: Lazy<Mutex<VideoState>> = Lazy::new(|| {
    Mutex::new(VideoState {
        wasm_initialized: false,
        is_muted: false,
    })
});

#[derive(Default)]
struct VideoState {
    wasm_initialized: bool,
    is_muted: bool,
}

fn get_video_element() -> Result<HtmlVideoElement, VideoError> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    let video_element = document
        .get_element_by_id("videoPlayer")
        .ok_or_else(|| VideoError::ElementNotFound("videoPlayer".to_string()))?
        .dyn_into::<HtmlVideoElement>()
        .map_err(|_| VideoError::ElementNotFound("videoPlayer".to_string()))?;
    Ok(video_element)
}

fn get_element_by_id(id: &str) -> Result<web_sys::Element, VideoError> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    document
        .get_element_by_id(id)
        .ok_or_else(|| VideoError::ElementNotFound(id.to_string()))
}

#[wasm_bindgen]
pub fn play_video() -> Result<(), JsValue> {
    Logger::info("Entering play_video()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let video_element = get_video_element()?;
    let is_video_playing = is_video_playing()?;
    
    if is_video_playing {
        Logger::info("Pausing video").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
        let _ = video_element.pause().map_err(|e| VideoError::VideoOperationFailed(format!("Failed to pause video: {:?}", e)))?;
        set_toggle_button_text("Play")?;
    } else {
        Logger::info("Playing video").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
        let _ = video_element.play().map_err(|e| VideoError::VideoOperationFailed(format!("Failed to play video: {:?}", e)))?;
        set_toggle_button_text("Pause")?;
    }
    
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn toggle_mute() -> Result<bool, JsValue> {
    Logger::info("Entering toggle_mute()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let video_element = get_video_element()?;
    let muted = !video_element.muted();
    
    Logger::info(&format!("Toggling mute state to: {}", muted))
        .map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    video_element.set_muted(muted);
    set_mute_button_text(muted)?;
    
    hide_error()?;
    Ok(video_element.muted())
}

#[wasm_bindgen]
pub fn toggle_fullscreen() -> Result<bool, JsValue> {
    Logger::info("Entering toggle_fullscreen()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    let video_element = get_video_element()?;
    
    if document.fullscreen_element().is_some() {
        Logger::info("Exiting fullscreen mode")
            .map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
        document.exit_fullscreen();
        hide_error()?;
        Ok(false)
    } else {
        Logger::info("Entering fullscreen mode")
            .map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
        video_element.request_fullscreen().map_err(|e| VideoError::VideoOperationFailed(format!("Failed to enter fullscreen: {:?}", e)))?;
        hide_error()?;
        Ok(true)
    }
}

#[wasm_bindgen]
pub fn get_video_time() -> Result<f64, JsValue> {
    Logger::info("Entering get_video_time()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let video_element = get_video_element()?;
    Ok(video_element.current_time())
}

#[wasm_bindgen]
pub fn get_video_duration() -> Result<f64, JsValue> {
    Logger::info("Entering get_video_duration()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let video_element = get_video_element()?;
    Ok(video_element.duration())
}

#[wasm_bindgen]
pub fn show_error(message: &str) -> Result<(), JsValue> {
    Logger::info("Entering show_error()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    Logger::error(message).map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let error_element = get_element_by_id("errorMessage")?;
    error_element.set_text_content(Some(message));
    error_element.set_attribute("style", "display: block").map_err(|e| VideoError::VideoOperationFailed(format!("Failed to show error: {:?}", e)))?;
    Ok(())
}

#[wasm_bindgen]
pub fn hide_error() -> Result<(), JsValue> {
    Logger::info("Entering hide_error()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let error_element = get_element_by_id("errorMessage")?;
    error_element.set_attribute("style", "display: none").map_err(|e| VideoError::VideoOperationFailed(format!("Failed to hide error: {:?}", e)))?;
    Ok(())
}

#[wasm_bindgen]
pub fn format_time(seconds: f64) -> String {
    Logger::info("Entering format_time()").map_err(|e| VideoError::VideoOperationFailed(e.to_string())).unwrap_or_default();
    let minutes = (seconds / 60.0).floor() as i32;
    let remaining_seconds = (seconds % 60.0).floor() as i32;
    format!("{}:{:02}", minutes, remaining_seconds)
}

#[wasm_bindgen]
pub fn is_video_playing() -> Result<bool, JsValue> {
    Logger::info("Entering is_video_playing()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let video_element = get_video_element()?;
    Ok(!video_element.paused())
}

#[wasm_bindgen]
pub fn is_video_muted() -> Result<bool, JsValue> {
    Logger::info("Entering is_video_muted()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let video_element = get_video_element()?;
    Ok(video_element.muted())
}

#[wasm_bindgen]
pub fn is_fullscreen() -> Result<bool, JsValue> {
    Logger::info("Entering is_fullscreen()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    Ok(document.fullscreen_element().is_some())
}

#[wasm_bindgen]
pub async fn update_time_display() -> Result<(), JsValue> {
    Logger::info("Entering update_time_display()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
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

#[wasm_bindgen]
pub fn get_wasm_initialized() -> bool {
    Logger::info("Entering get_wasm_initialized()").map_err(|e| VideoError::VideoOperationFailed(e.to_string())).unwrap_or_default();
    VIDEO_STATE.lock().unwrap().wasm_initialized
}

#[wasm_bindgen]
pub fn set_wasm_initialized(value: bool) -> Result<(), JsValue> {
    Logger::info("Entering set_wasm_initialized()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let mut result = VIDEO_STATE.lock().map_err(|e| VideoError::StateError(format!("Failed to lock state: {:?}", e)))?;
    if result.wasm_initialized != value {
        result.wasm_initialized = value;
    }
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn get_is_muted() -> bool {
    Logger::info("Entering get_is_muted()").map_err(|e| VideoError::VideoOperationFailed(e.to_string())).unwrap_or_default();
    VIDEO_STATE.lock().unwrap().is_muted
}

#[wasm_bindgen]
pub fn set_is_muted(value: bool) -> Result<(), JsValue> {
    Logger::info("Entering set_is_muted()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let mut result = VIDEO_STATE.lock().map_err(|e| VideoError::StateError(format!("Failed to lock state: {:?}", e)))?;
    if result.is_muted != value {
        result.is_muted = value;
    }
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_toggle_button_disabled(disabled: bool) -> Result<(), JsValue> {
    Logger::info("Entering set_toggle_button_disabled()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let button = get_element_by_id("toggleButton")?;
    button.set_attribute("disabled", if disabled { "true" } else { "false" })
        .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to set button disabled state: {:?}", e)))?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_toggle_button_text(text: &str) -> Result<(), JsValue> {
    Logger::info("Entering set_toggle_button_text()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let button = get_element_by_id("toggleButton")?;
    button.set_text_content(Some(text));
    Ok(())
}

#[wasm_bindgen]
pub fn set_mute_button_text(is_muted: bool) -> Result<(), JsValue> {
    Logger::info("Entering set_mute_button_text()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let button = get_element_by_id("muteButton")?;
    button.set_text_content(Some(if is_muted { "Unmute" } else { "Mute" }));
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_fullscreen_button_text() -> Result<(), JsValue> {
    Logger::info("Entering set_fullscreen_button_text()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let is_fullscreen = toggle_fullscreen()?;
    let button = get_element_by_id("fullscreenButton")?;
    button.set_text_content(Some(if is_fullscreen { "Exit Fullscreen" } else { "Fullscreen" }));
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn update_fullscreen_button_text() -> Result<(), JsValue> {
    Logger::info("Entering update_fullscreen_button_text()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let is_fullscreen = is_fullscreen()?;
    let button = get_element_by_id("fullscreenButton")?;
    button.set_text_content(Some(if is_fullscreen { "Exit Fullscreen" } else { "Fullscreen" }));
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn update_mute_button_text() -> Result<(), JsValue> {
    Logger::info("Entering update_mute_button_text()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let is_muted = is_video_muted()?;
    let button = get_element_by_id("muteButton")?;
    button.set_text_content(Some(if is_muted { "Unmute" } else { "Mute" }));
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn seek_video(percentage: f64) -> Result<(), JsValue> {
    Logger::info("Entering seek_video()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let video_element = get_video_element()?;
    let duration = video_element.duration();
    if !duration.is_nan() {
        let time = duration * (percentage / 100.0);
        video_element.set_current_time(time);
    }
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn update_seek_bar() -> Result<(), JsValue> {
    Logger::info("Entering update_seek_bar()").map_err(|e| VideoError::VideoOperationFailed(e.to_string()))?;
    let video_element = get_video_element()?;
    let duration = video_element.duration();
    if !duration.is_nan() {
        let current_time = video_element.current_time();
        let value = (current_time / duration) * 100.0;
        let seek_bar = get_element_by_id("seekBar")?;
        seek_bar.set_attribute("value", &value.to_string())
            .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to update seek bar: {:?}", e)))?;
    }
    hide_error()?;
    Ok(())
} 