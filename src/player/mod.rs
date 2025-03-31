use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;
use crate::player::error::{ VideoError};
use crate::player::event_listeners::setup_event_listeners;
mod dom;
pub use dom::{get_video_element, get_element_by_id};

#[wasm_bindgen]
pub fn init_video_player() -> Result<(), VideoError> {
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