use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;
use std::sync::Mutex;
use once_cell::sync::Lazy;

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

#[wasm_bindgen]
pub fn play_video() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    let is_video_playing = is_video_playing()?;
    if is_video_playing {
        let _ = video_element.pause()?;
        let _result = set_toggle_button_text("Play");
    } else {
        let _ = video_element.play()?;
        let _result = set_toggle_button_text("Pause");
    }
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn toggle_mute() -> Result<bool, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    let muted = video_element.muted();
    video_element.set_muted(!muted);
    if muted {
        let _result = set_mute_button_text(false);
    } else {
        let _result = set_mute_button_text(true);
    }
    
    hide_error()?;
    Ok(video_element.muted())
}

#[wasm_bindgen]
pub fn toggle_fullscreen() -> Result<bool, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    if document.fullscreen_element().is_some() {
        document.exit_fullscreen();
        hide_error()?;
        Ok(false)
    } else {
        video_element.request_fullscreen()?;
        hide_error()?;
        Ok(true)
    }
}

#[wasm_bindgen]
pub fn get_video_time() -> Result<f64, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    Ok(video_element.current_time())
}

#[wasm_bindgen]
pub fn get_video_duration() -> Result<f64, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    Ok(video_element.duration())
}

#[wasm_bindgen]
pub fn show_error(message: &str) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let error_element = document.get_element_by_id("errorMessage").unwrap();
    
    error_element.set_text_content(Some(message));
    error_element.set_attribute("style", "display: block")?;
    Ok(())
}

#[wasm_bindgen]
pub fn hide_error() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let error_element = document.get_element_by_id("errorMessage").unwrap();
    
    error_element.set_attribute("style", "display: none")?;
    Ok(())
}

#[wasm_bindgen]
pub fn format_time(seconds: f64) -> String {
    let minutes = (seconds / 60.0).floor() as i32;
    let remaining_seconds = (seconds % 60.0).floor() as i32;
    format!("{}:{:02}", minutes, remaining_seconds)
}

#[wasm_bindgen]
pub fn is_video_playing() -> Result<bool, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    Ok(!video_element.paused())
}

#[wasm_bindgen]
pub fn is_video_muted() -> Result<bool, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let video_element = document
        .get_element_by_id("videoPlayer")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()?;
    
    Ok(video_element.muted())
}

#[wasm_bindgen]
pub fn is_fullscreen() -> Result<bool, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    Ok(document.fullscreen_element().is_some())
}

#[wasm_bindgen]
pub async fn update_time_display() -> Result<(), JsValue> {
    let current_time = get_video_time()?;
    let duration = get_video_duration()?;
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let current_time_display = document.get_element_by_id("currentTime").unwrap();
    let total_time_display = document.get_element_by_id("totalTime").unwrap();
    
    current_time_display.set_text_content(Some(&format_time(current_time)));
    total_time_display.set_text_content(Some(&format_time(duration)));
    
    Ok(())
}

#[wasm_bindgen]
pub fn get_wasm_initialized() -> bool {
    VIDEO_STATE.lock().unwrap().wasm_initialized
}

#[wasm_bindgen]
pub fn set_wasm_initialized(value: bool) {
    VIDEO_STATE.lock().unwrap().wasm_initialized = value;
    hide_error().unwrap();
}

#[wasm_bindgen]
pub fn get_is_muted() -> bool {
    VIDEO_STATE.lock().unwrap().is_muted
}

#[wasm_bindgen]
pub fn set_is_muted(value: bool) {
    VIDEO_STATE.lock().unwrap().is_muted = value;
}

#[wasm_bindgen]
pub fn set_toggle_button_disabled(disabled: bool) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let button = document.get_element_by_id("toggleButton").unwrap();
    
    button.set_attribute("disabled", if disabled { "true" } else { "false" })?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_toggle_button_text(text: &str) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let button = document.get_element_by_id("toggleButton").unwrap();
    
    button.set_text_content(Some(text));
    Ok(())
}

#[wasm_bindgen]
pub fn set_mute_button_text(is_muted: bool) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let button = document.get_element_by_id("muteButton").unwrap();
    
    button.set_text_content(Some(if is_muted { "Unmute" } else { "Mute" }));
    hide_error()?;
    Ok(())
}

#[wasm_bindgen]
pub fn set_fullscreen_button_text() -> Result<(), JsValue> {
    let is_fullscreen = toggle_fullscreen()?;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let button = document.get_element_by_id("fullscreenButton").unwrap();
    
    button.set_text_content(Some(if is_fullscreen { "Exit Fullscreen" } else { "Fullscreen" }));
    hide_error()?;
    Ok(())
} 