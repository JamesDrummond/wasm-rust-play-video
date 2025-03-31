use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static VIDEO_STATE: Lazy<Mutex<VideoState>> = Lazy::new(|| {
    Mutex::new(VideoState {
        is_muted: false,
        playback_speed: 1.0,
    })
});

#[derive(Clone)]
pub struct VideoState {
    pub is_muted: bool,
    pub playback_speed: f64,
} 