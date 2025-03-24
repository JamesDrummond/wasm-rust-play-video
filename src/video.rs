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
        playback_speed: 1.0,
    })
});

#[derive(Default)]
struct VideoState {
    wasm_initialized: bool,
    is_muted: bool,
    playback_speed: f64,
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
        set_toggle_button_text("Play")?;
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
        set_toggle_button_text("Pause")?;
    }
    
    hide_error()?;
    Ok(())
}

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

#[wasm_bindgen]
pub fn format_time(seconds: f64) -> String {
    Logger::info("Entering format_time()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    }).unwrap_or_default();
    let minutes = (seconds / 60.0).floor() as i32;
    let remaining_seconds = (seconds % 60.0).floor() as i32;
    format!("{}:{:02}", minutes, remaining_seconds)
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
pub async fn update_time_display() -> Result<(), JsValue> {
    Logger::info("Entering update_time_display()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
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

#[wasm_bindgen]
pub fn set_toggle_button_disabled(disabled: bool) -> Result<(), JsValue> {
    Logger::info("Entering set_toggle_button_disabled()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let button = get_element_by_id("toggleButton")?;
    button.set_attribute("disabled", if disabled { "true" } else { "false" })
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to set button disabled state: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_toggle_button_text(text: &str) -> Result<(), JsValue> {
    Logger::info("Entering set_toggle_button_text()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let button = get_element_by_id("toggleButton")?;
    button.set_text_content(Some(text));
    Ok(())
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
pub fn seek_video(percentage: f64) -> Result<(), JsValue> {
    Logger::info("Entering seek_video()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
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
    Logger::info("Entering update_seek_bar()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let video_element = get_video_element()?;
    let duration = video_element.duration();
    if !duration.is_nan() {
        let current_time = video_element.current_time();
        let value = (current_time / duration) * 100.0;
        let seek_bar = get_element_by_id("seekBar")?;
        seek_bar.set_attribute("value", &value.to_string())
            .map_err(|e| {
                let error = VideoError::VideoOperationFailed(format!("Failed to update seek bar: {:?}", e));
                show_error(&error.to_string()).unwrap_or_default();
                error
            })?;
    }
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub async fn download_video() -> Result<(), JsValue> {
    Logger::info("Entering download_video()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    hide_menus()?;
    let video_element = get_video_element()?;
    let source = video_element.query_selector("source")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to get source element: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?
        .ok_or_else(|| {
            let error = VideoError::VideoOperationFailed("No source element found".to_string());
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    let video_url = source.get_attribute("src")
        .ok_or_else(|| {
            let error = VideoError::VideoOperationFailed("No source URL found".to_string());
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
    
    // Create fetch request
    let init = web_sys::RequestInit::new();
    init.set_method("GET");
    init.set_mode(web_sys::RequestMode::Cors);
    
    let request = web_sys::Request::new_with_str_and_init(&video_url, &init)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to create request: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Fetch the video
    let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to fetch video: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    let response: web_sys::Response = response.dyn_into()
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to convert response: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Get the blob
    let blob = wasm_bindgen_futures::JsFuture::from(response.blob().unwrap())
        .await
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to get blob: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    let blob: web_sys::Blob = blob.dyn_into()
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to convert blob: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Create object URL
    let url = web_sys::Url::create_object_url_with_blob(&blob)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to create object URL: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Create anchor element
    let anchor = document.create_element("a")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to create anchor element: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Set download attributes
    anchor.set_attribute("href", &url)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to set href: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    anchor.set_attribute("download", "video.mp4")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to set download attribute: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Append to document and click
    document.body()
        .ok_or_else(|| {
            let error = VideoError::VideoOperationFailed("No body element found".to_string());
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?
        .append_child(&anchor)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to append anchor: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Click the anchor element
    let html_anchor = anchor.unchecked_into::<web_sys::HtmlElement>();
    html_anchor.click();
    
    // Clean up
    web_sys::Url::revoke_object_url(&url)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to revoke object URL: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    document.body()
        .ok_or_else(|| {
            let error = VideoError::VideoOperationFailed("No body element found".to_string());
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?
        .remove_child(&html_anchor)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to remove anchor: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    hide_error()?;
    Ok(())
}

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

#[wasm_bindgen]
pub fn hide_menus() -> Result<(), JsValue> {
    Logger::info("Entering hide_menus()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let playback_speed_menu = get_element_by_id("playbackSpeedMenu")?;
    let context_menu = get_element_by_id("contextMenu")?;
    
    playback_speed_menu.set_attribute("class", "playback-speed-menu")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to hide playback speed menu: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    context_menu.set_attribute("class", "context-menu")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to hide context menu: {:?}", e));
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