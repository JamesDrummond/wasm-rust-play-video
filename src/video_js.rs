use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlElement, 
    HtmlVideoElement, 
    Window, 
    Document,
    Element,
};
use crate::video::{VideoError, VIDEO_STATE};
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
pub fn setup_event_listeners() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    
    let video_player = document
        .get_element_by_id("videoPlayer")
        .ok_or(VideoError::ElementNotFound("videoPlayer".to_string()))?
        .dyn_into::<HtmlVideoElement>()?;

    let seek_bar = document
        .get_element_by_id("seekBar")
        .ok_or(VideoError::ElementNotFound("seekBar".to_string()))?
        .dyn_into::<HtmlElement>()?;

    // Time update event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            let update_time = update_time_display();
            spawn_local(async move {
                update_time.await.unwrap_or_default();
            });
        }) as Box<dyn FnMut()>);
        video_player.add_event_listener_with_callback(
            "timeupdate",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Metadata loaded event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            let update_time = update_time_display();
            spawn_local(async move {
                update_time.await.unwrap_or_default();
            });
        }) as Box<dyn FnMut()>);
        video_player.add_event_listener_with_callback(
            "loadedmetadata",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Duration change event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            let update_time = update_time_display();
            spawn_local(async move {
                update_time.await.unwrap_or_default();
            });
        }) as Box<dyn FnMut()>);
        video_player.add_event_listener_with_callback(
            "durationchange",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Seek bar update event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            update_seek_bar().unwrap_or_default();
        }) as Box<dyn FnMut()>);
        video_player.add_event_listener_with_callback(
            "timeupdate",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Play event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            set_toggle_button_text("Pause").unwrap_or_default();
        }) as Box<dyn FnMut()>);
        video_player.add_event_listener_with_callback(
            "play",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Pause event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            set_toggle_button_text("Play").unwrap_or_default();
        }) as Box<dyn FnMut()>);
        video_player.add_event_listener_with_callback(
            "pause",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Volume change event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            update_mute_button_text().unwrap_or_default();
        }) as Box<dyn FnMut()>);
        video_player.add_event_listener_with_callback(
            "volumechange",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Seek bar input event listener
    {
        let seek_bar_value = seek_bar.get_attribute("value").unwrap_or_default();
        let closure = Closure::wrap(Box::new(move || {
            if let Ok(value) = seek_bar_value.parse::<f64>() {
                seek_video(value).unwrap_or_default();
            }
        }) as Box<dyn FnMut()>);
        seek_bar.add_event_listener_with_callback(
            "input",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Fullscreen change event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            update_fullscreen_button_text().unwrap_or_default();
        }) as Box<dyn FnMut()>);
        document.add_event_listener_with_callback(
            "fullscreenchange",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    Ok(())
}

#[wasm_bindgen]
pub fn init_wasm() -> Result<(), JsValue> {
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

// Re-export functions from video.rs
pub use crate::video::{
    play_video,
    toggle_mute,
    toggle_fullscreen,
    get_video_time,
    get_video_duration,
    show_error,
    format_time,
    is_video_playing,
    is_video_muted,
    is_fullscreen,
    update_time_display,
    get_wasm_initialized,
    set_wasm_initialized,
    get_is_muted,
    set_is_muted,
    set_toggle_button_disabled,
    set_toggle_button_text,
    set_mute_button_text,
    set_fullscreen_button_text,
    update_mute_button_text,
    update_fullscreen_button_text,
    seek_video,
    update_seek_bar,
    download_video,
    get_playback_speed,
    set_playback_speed,
    get_current_playback_speed,
    update_playback_speed_active_state,
}; 