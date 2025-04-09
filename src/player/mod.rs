use wasm_bindgen::prelude::*;
use crate::player::error::VideoError;
use crate::player::event_listeners::setup_event_listeners;
mod dom;
pub use dom::{get_video_element, get_element_by_id,add_video_source};
mod element_ids;
pub use element_ids::ElementIds;

#[wasm_bindgen]
pub fn init_video_player(element_ids: ElementIds) -> Result<(), VideoError> {
    add_video_source("https://storage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4", "video/mp4").map_err(|e| 
        VideoError::VideoOperationFailed(format!("Failed to add video source: {:?}", e)))?;
    setup_event_listeners(element_ids).map_err(|e| 
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