use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlVideoElement,
    HtmlElement
};
use crate::logger::Logger;
use crate::player::error::{show_error, hide_error, VideoError};
use crate::player::state::VIDEO_STATE;
use crate::player::event_listeners::setup_event_listeners;
use crate::player::playback_speed::{get_playback_speed, set_playback_speed, get_current_playback_speed, update_playback_speed_active_state};

pub fn get_video_element() -> Result<HtmlVideoElement, VideoError> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    let video_element = document
        .get_element_by_id("videoPlayer")
        .ok_or_else(|| VideoError::ElementNotFound("videoPlayer".to_string()))?
        .dyn_into::<HtmlVideoElement>()
        .map_err(|_| VideoError::ElementNotFound("videoPlayer".to_string()))?;
    Ok(video_element)
}

pub fn get_element_by_id(id: &str) -> Result<web_sys::Element, VideoError> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    document
        .get_element_by_id(id)
        .ok_or_else(|| VideoError::ElementNotFound(id.to_string()))
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
pub fn init_video() -> Result<(), JsValue> {
    setup_event_listeners()?;
    set_wasm_initialized(true)?;
    Ok(())
}

#[wasm_bindgen]
pub fn toggle_context_menu(x: f64, y: f64) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    
    let context_menu = document
        .get_element_by_id("contextMenu")
        .ok_or(VideoError::ElementNotFound("contextMenu".to_string()))?
        .dyn_into::<HtmlElement>()?;

    context_menu.set_attribute("style", &format!("left: {}px; top: {}px;", x, y))?;
    context_menu.set_attribute("class", "context-menu show")?;
    Ok(())
}

#[wasm_bindgen]
pub fn toggle_playback_speed_menu(x: f64, y: f64) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    
    let playback_speed_menu = document
        .get_element_by_id("playbackSpeedMenu")
        .ok_or(VideoError::ElementNotFound("playbackSpeedMenu".to_string()))?
        .dyn_into::<HtmlElement>()?;

    playback_speed_menu.set_attribute("style", &format!("left: {}px; top: {}px;", x, y))?;
    playback_speed_menu.set_attribute("class", "playback-speed-menu show")?;
    Ok(())
}

#[wasm_bindgen]
pub fn toggle_picture_in_picture() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    
    let video_player = document
        .get_element_by_id("videoPlayer")
        .ok_or(VideoError::ElementNotFound("videoPlayer".to_string()))?
        .dyn_into::<HtmlVideoElement>()?;

    // Use JavaScript interop for Picture-in-Picture API
    let js_value = js_sys::Reflect::get(
        &window,
        &JsValue::from_str("document"),
    )?;
    let document = js_value.dyn_into::<web_sys::Document>()?;
    
    let pip_element = js_sys::Reflect::get(
        &document,
        &JsValue::from_str("pictureInPictureElement"),
    )?;

    if !pip_element.is_null() {
        let exit_pip = js_sys::Reflect::get(
            &document,
            &JsValue::from_str("exitPictureInPicture"),
        )?;
        let exit_pip_fn = exit_pip.dyn_into::<js_sys::Function>()?;
        // Call the function with document as this context
        exit_pip_fn.call1(&document, &JsValue::null())?;
    } else {
        let pip_enabled = js_sys::Reflect::get(
            &document,
            &JsValue::from_str("pictureInPictureEnabled"),
        )?;
        if pip_enabled.as_bool().unwrap_or(false) {
            // Get the requestPictureInPicture function
            let request_pip = js_sys::Reflect::get(
                &video_player,
                &JsValue::from_str("requestPictureInPicture"),
            )?;
            let request_pip_fn = request_pip.dyn_into::<js_sys::Function>()?;
            // Call the function with video_player as this context
            request_pip_fn.call1(&video_player, &JsValue::null())?;
        }
    }
    Ok(())
} 