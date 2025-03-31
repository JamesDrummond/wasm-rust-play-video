use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct ElementIds {
    pub(crate) video_player: String,
    pub(crate) toggle_button: String,
    pub(crate) mute_button: String,
    pub(crate) fullscreen_button: String,
    pub(crate) menu_button: String,
    pub(crate) current_time: String,
    pub(crate) total_time: String,
    pub(crate) error_message: String,
    pub(crate) context_menu: String,
    pub(crate) playback_speed_menu: String,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ElementClasses {
    pub(crate) context_menu: String,
    pub(crate) context_menu_show: String,
    pub(crate) playback_speed_menu: String,
    pub(crate) playback_speed_menu_show: String,
    pub(crate) speed_option: String,
    pub(crate) context_menu_item: String,
}

#[wasm_bindgen]
impl ElementClasses {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ElementClasses {
        ElementClasses {
            context_menu: "context-menu".to_string(),
            context_menu_show: "context-menu show".to_string(),
            playback_speed_menu: "playback-speed-menu".to_string(),
            playback_speed_menu_show: "playback-speed-menu show".to_string(),
            speed_option: "speed-option".to_string(),
            context_menu_item: "context-menu-item".to_string(),
        }
    }

    pub fn context_menu(&self) -> String {
        self.context_menu.clone()
    }

    pub fn context_menu_show(&self) -> String {
        self.context_menu_show.clone()
    }

    pub fn playback_speed_menu(&self) -> String {
        self.playback_speed_menu.clone()
    }

    pub fn playback_speed_menu_show(&self) -> String {
        self.playback_speed_menu_show.clone()
    }

    pub fn speed_option(&self) -> String {
        self.speed_option.clone()
    }

    pub fn context_menu_item(&self) -> String {
        self.context_menu_item.clone()
    }
}

#[wasm_bindgen]
impl ElementIds {
    #[wasm_bindgen(constructor)]
    pub fn new(
        video_player: String,
        toggle_button: String,
        mute_button: String,
        fullscreen_button: String,
        menu_button: String,
        current_time: String,
        total_time: String,
        error_message: String,
        context_menu: String,
        playback_speed_menu: String,
    ) -> ElementIds {
        ElementIds {
            video_player,
            toggle_button,
            mute_button,
            fullscreen_button,
            menu_button,
            current_time,
            total_time,
            error_message,
            context_menu,
            playback_speed_menu,
        }
    }

    pub fn video_player(&self) -> String {
        self.video_player.to_string()
    }

    pub fn toggle_button(&self) -> String {
        self.toggle_button.to_string()
    }

    pub fn mute_button(&self) -> String {
        self.mute_button.to_string()
    }

    pub fn fullscreen_button(&self) -> String {
        self.fullscreen_button.to_string()
    }

    pub fn menu_button(&self) -> String {
        self.menu_button.to_string()
    }

    pub fn current_time(&self) -> String {
        self.current_time.to_string()
    }

    pub fn total_time(&self) -> String {
        self.total_time.to_string()
    }

    pub fn error_message(&self) -> String {
        self.error_message.to_string()
    }

    pub fn context_menu(&self) -> String {
        self.context_menu.to_string()
    }

    pub fn playback_speed_menu(&self) -> String {
        self.playback_speed_menu.to_string()
    }
} 