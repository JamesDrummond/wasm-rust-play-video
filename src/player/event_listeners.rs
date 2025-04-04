use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlVideoElement,
    Event,
};
use wasm_bindgen_futures::spawn_local;
use crate::player::play_pause::{play_video, set_toggle_play};
use crate::player::mute::{toggle_mute, update_mute_button_text};
use crate::player::fullscreen::{toggle_fullscreen, update_fullscreen_button_text};
use crate::player::error::VideoError;
use crate::player::time::update_time_display;
use crate::player::download::download_video;
use crate::player::menu::{position_playback_speed_menu, position_context_menu};
use crate::player::picture_in_picture::toggle_picture_in_picture;
use crate::player::playback_speed::set_playback_speed;
use crate::player::ElementIds;
use crate::player::element_ids::ElementClasses;

// Event name constants
const EVENT_CLICK: &str = "click";
const EVENT_TIMEUPDATE: &str = "timeupdate";
const EVENT_LOADEDMETADATA: &str = "loadedmetadata";
const EVENT_DURATIONCHANGE: &str = "durationchange";
const EVENT_PLAY: &str = "play";
const EVENT_PAUSE: &str = "pause";
const EVENT_VOLUMECHANGE: &str = "volumechange";
const EVENT_FULLSCREENCHANGE: &str = "fullscreenchange";

// Button text constants
const BUTTON_TEXT_PLAY: &str = "Play";
const BUTTON_TEXT_PAUSE: &str = "Pause";

// Error message constants
const ERROR_DOWNLOAD_BUTTON_NOT_FOUND: &str = "download button";
const ERROR_PLAYBACK_SPEED_BUTTON_NOT_FOUND: &str = "playback speed button";
const ERROR_PIP_BUTTON_NOT_FOUND: &str = "pip button";
const ERROR_SPEED_OPTION_NOT_FOUND: &str = "Failed to get speed option";
const ERROR_NODE_TO_ELEMENT_CONVERSION: &str = "Failed to convert Node to Element";
const ERROR_NO_TEXT_CONTENT: &str = "No text content found";
const ERROR_SPEED_PARSE: &str = "Failed to parse speed";

#[wasm_bindgen]
pub fn setup_event_listeners(element_ids: ElementIds) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    
    let video_player = document
        .get_element_by_id(&element_ids.video_player())
        .ok_or(VideoError::ElementNotFound(element_ids.video_player()))?
        .dyn_into::<HtmlVideoElement>()?;

    // Get menu elements
    let context_menu = document
        .get_element_by_id(&element_ids.context_menu())
        .ok_or(VideoError::ElementNotFound(element_ids.context_menu()))?;
    
    let playback_speed_menu = document
        .get_element_by_id(&element_ids.playback_speed_menu())
        .ok_or(VideoError::ElementNotFound(element_ids.playback_speed_menu()))?;

    let element_classes = ElementClasses::new();

    // Click outside listener to close menus
    {
        let context_menu_clone = context_menu.clone();
        let playback_speed_menu_clone = playback_speed_menu.clone();
        let element_classes_clone = element_classes.clone();
        
        let closure = Closure::wrap(Box::new(move |event: Event| {
            if let Some(target) = event.target() {
                if let Ok(target_element) = target.dyn_into::<web_sys::Element>() {
                    if !context_menu_clone.contains(Some(&target_element)) && 
                       !playback_speed_menu_clone.contains(Some(&target_element)) {
                        context_menu_clone.set_attribute("class", &element_classes_clone.context_menu()).unwrap_or_default();
                        playback_speed_menu_clone.set_attribute("class", &element_classes_clone.playback_speed_menu()).unwrap_or_default();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        document.add_event_listener_with_callback(
            EVENT_CLICK,
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Time update event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            let update_time = update_time_display();
            spawn_local(async move {
                update_time.await.unwrap_or_default();
            });
        }) as Box<dyn FnMut()>);
        video_player.add_event_listener_with_callback(
            EVENT_TIMEUPDATE,
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
            EVENT_LOADEDMETADATA,
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
            EVENT_DURATIONCHANGE,
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Play event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            let set_text = set_toggle_play(BUTTON_TEXT_PAUSE);
            spawn_local(async move {
                set_text.await.unwrap_or_default();
            });
        }) as Box<dyn FnMut()>);
        video_player.add_event_listener_with_callback(
            EVENT_PLAY,
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Pause event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            let set_text = set_toggle_play(BUTTON_TEXT_PLAY);
            spawn_local(async move {
                set_text.await.unwrap_or_default();
            });
        }) as Box<dyn FnMut()>);
        video_player.add_event_listener_with_callback(
            EVENT_PAUSE,
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
            EVENT_VOLUMECHANGE,
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
            EVENT_FULLSCREENCHANGE,
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Toggle button click event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            let play_video = play_video();
            spawn_local(async move {
                play_video.await.unwrap_or_default();
            });
        }) as Box<dyn FnMut()>);
        
        let toggle_button = document
            .get_element_by_id(&element_ids.toggle_button())
            .ok_or(VideoError::ElementNotFound(element_ids.toggle_button()))?;
            
        toggle_button.add_event_listener_with_callback(
            EVENT_CLICK,
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Mute button click event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            toggle_mute().unwrap_or_default();
        }) as Box<dyn FnMut()>);
        
        let mute_button = document
            .get_element_by_id(&element_ids.mute_button())
            .ok_or(VideoError::ElementNotFound(element_ids.mute_button()))?;
            
        mute_button.add_event_listener_with_callback(
            EVENT_CLICK,
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Fullscreen button click event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            toggle_fullscreen().unwrap_or_default();
        }) as Box<dyn FnMut()>);
        
        let fullscreen_button = document
            .get_element_by_id(&element_ids.fullscreen_button())
            .ok_or(VideoError::ElementNotFound(element_ids.fullscreen_button()))?;
            
        fullscreen_button.add_event_listener_with_callback(
            EVENT_CLICK,
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Context menu download button click event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            let download = download_video();
            spawn_local(async move {
                download.await.unwrap_or_default();
            });
        }) as Box<dyn FnMut()>);
        
        let download_button = document
            .get_element_by_id(&element_ids.context_menu())
            .ok_or(VideoError::ElementNotFound(element_ids.context_menu()))?
            .query_selector(&format!(".{}", element_classes.context_menu_item()))
            .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to get download button: {:?}", e)))?
            .ok_or(VideoError::ElementNotFound(ERROR_DOWNLOAD_BUTTON_NOT_FOUND.to_string()))?;
            
        download_button.add_event_listener_with_callback(
            EVENT_CLICK,
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Context menu playback speed button click event listener
    {
        let closure = Closure::wrap(Box::new(move |event: Event| {
            if let Some(target) = event.target() {
                if let Ok(target_element) = target.dyn_into::<web_sys::Element>() {
                    let rect = target_element.get_bounding_client_rect();
                    let _ = position_playback_speed_menu(rect.right(), rect.bottom());
                }
            }
        }) as Box<dyn FnMut(Event)>);
        
        let playback_speed_button = document
            .get_element_by_id(&element_ids.context_menu())
            .ok_or(VideoError::ElementNotFound(element_ids.context_menu()))?
            .query_selector_all(&format!(".{}", element_classes.context_menu_item()))
            .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to get playback speed button: {:?}", e)))?
            .get(1)
            .ok_or(VideoError::ElementNotFound(ERROR_PLAYBACK_SPEED_BUTTON_NOT_FOUND.to_string()))?;
            
        playback_speed_button.add_event_listener_with_callback(
            EVENT_CLICK,
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Context menu picture-in-picture button click event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            toggle_picture_in_picture().unwrap_or_default();
        }) as Box<dyn FnMut()>);
        
        let pip_button = document
            .get_element_by_id(&element_ids.context_menu())
            .ok_or(VideoError::ElementNotFound(element_ids.context_menu()))?
            .query_selector_all(&format!(".{}", element_classes.context_menu_item()))
            .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to get pip button: {:?}", e)))?
            .get(2)
            .ok_or(VideoError::ElementNotFound(ERROR_PIP_BUTTON_NOT_FOUND.to_string()))?;
            
        pip_button.add_event_listener_with_callback(
            EVENT_CLICK,
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Speed options click event listeners
    {
        let speed_options = playback_speed_menu.query_selector_all(&format!(".{}", element_classes.speed_option()))
            .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to get speed options: {:?}", e)))?;
        
        for i in 0..speed_options.length() {
            let option = speed_options.get(i)
                .ok_or_else(|| VideoError::ElementNotFound(ERROR_SPEED_OPTION_NOT_FOUND.to_string()))?
                .dyn_into::<web_sys::Element>()
                .map_err(|e| VideoError::VideoOperationFailed(format!("{}: {:?}", ERROR_NODE_TO_ELEMENT_CONVERSION, e)))?;
            
            let text_content = option.text_content()
                .ok_or_else(|| VideoError::VideoOperationFailed(ERROR_NO_TEXT_CONTENT.to_string()))?;
            
            let speed = text_content.replace('x', "").parse::<f64>()
                .map_err(|e| VideoError::VideoOperationFailed(format!("{}: {:?}", ERROR_SPEED_PARSE, e)))?;
            
            let closure = Closure::wrap(Box::new(move || {
                set_playback_speed(speed).unwrap_or_default();
            }) as Box<dyn FnMut()>);
            
            option.add_event_listener_with_callback(
                EVENT_CLICK,
                closure.as_ref().unchecked_ref(),
            )?;
            closure.forget();
        }
    }

    // Menu button click event listener
    {
        let closure = Closure::wrap(Box::new(move |event: Event| {
            if let Some(mouse_event) = event.dyn_into::<web_sys::MouseEvent>().ok() {
                // Stop event propagation
                mouse_event.stop_propagation();
                // Show context menu at button position
                let _ = position_context_menu(mouse_event.client_x() as f64, mouse_event.client_y() as f64);
            }
        }) as Box<dyn FnMut(Event)>);
        
        let menu_button = document
            .get_element_by_id(&element_ids.menu_button())
            .ok_or(VideoError::ElementNotFound(element_ids.menu_button()))?;
            
        menu_button.add_event_listener_with_callback(
            EVENT_CLICK,
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    Ok(())
} 