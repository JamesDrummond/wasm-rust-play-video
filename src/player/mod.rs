use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;
use crate::player::error::{ VideoError};
use crate::player::event_listeners::setup_event_listeners;

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
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| VideoError::ElementNotFound(id.to_string()))?;
    Ok(element)
}

#[wasm_bindgen]
pub fn init_video_player() -> Result<(), VideoError> {
    let _video_element = get_video_element()?;
    setup_event_listeners().map_err(|e| 
        VideoError::VideoOperationFailed(format!("Failed to setup event listeners: {:?}", e)))?;
    Ok(())
}

pub mod error;
pub mod fullscreen;
pub mod menu;
pub mod mute;
pub mod picture_in_picture;
pub mod play_pause;
pub mod state;
pub mod time;
pub mod download;
pub mod event_listeners;
pub mod playback_speed;