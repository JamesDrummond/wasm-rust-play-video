pub mod error;
pub mod fullscreen;
pub mod mute;
pub mod play_pause;
pub mod state;
pub mod time;
pub mod video;
pub mod download;

pub use error::VideoError;
pub use fullscreen::toggle_fullscreen;
pub use mute::toggle_mute;
pub use play_pause::play_video;
pub use state::VIDEO_STATE;
pub use time::update_time_display;
pub use video::{
    get_video_element,
    get_element_by_id,
    get_wasm_initialized,
    set_wasm_initialized,
    get_is_muted,
    get_playback_speed,
    set_playback_speed,
    get_current_playback_speed,
    update_playback_speed_active_state,
    hide_menus,
    position_playback_speed_menu,
    position_context_menu,
    setup_event_listeners,
    init_video,
    toggle_context_menu,
    toggle_playback_speed_menu,
    toggle_picture_in_picture,
};
pub use download::download_video;