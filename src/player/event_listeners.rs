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

    // Click outside listener to close menus
    {
        let context_menu_clone = context_menu.clone();
        let playback_speed_menu_clone = playback_speed_menu.clone();
        
        let closure = Closure::wrap(Box::new(move |event: Event| {
            if let Some(target) = event.target() {
                if let Ok(target_element) = target.dyn_into::<web_sys::Element>() {
                    if !context_menu_clone.contains(Some(&target_element)) && 
                       !playback_speed_menu_clone.contains(Some(&target_element)) {
                        context_menu_clone.set_attribute("class", "context-menu").unwrap_or_default();
                        playback_speed_menu_clone.set_attribute("class", "playback-speed-menu").unwrap_or_default();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        document.add_event_listener_with_callback(
            "click",
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

    // Play event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            let set_text = set_toggle_play("Pause");
            spawn_local(async move {
                set_text.await.unwrap_or_default();
            });
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
            let set_text = set_toggle_play("Play");
            spawn_local(async move {
                set_text.await.unwrap_or_default();
            });
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

    // Toggle button click event listener
    {
        let closure = Closure::wrap(Box::new(move || {
            let play_video = play_video();
            spawn_local(async move {
                play_video.await.unwrap_or_default();
            });
        }) as Box<dyn FnMut()>);
        
        let toggle_button = document
            .get_element_by_id("toggleButton")
            .ok_or(VideoError::ElementNotFound("toggleButton".to_string()))?;
            
        toggle_button.add_event_listener_with_callback(
            "click",
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
            .get_element_by_id("muteButton")
            .ok_or(VideoError::ElementNotFound("muteButton".to_string()))?;
            
        mute_button.add_event_listener_with_callback(
            "click",
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
            .get_element_by_id("fullscreenButton")
            .ok_or(VideoError::ElementNotFound("fullscreenButton".to_string()))?;
            
        fullscreen_button.add_event_listener_with_callback(
            "click",
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
            .get_element_by_id("contextMenu")
            .ok_or(VideoError::ElementNotFound("contextMenu".to_string()))?
            .query_selector(".context-menu-item")
            .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to get download button: {:?}", e)))?
            .ok_or(VideoError::ElementNotFound("download button".to_string()))?;
            
        download_button.add_event_listener_with_callback(
            "click",
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
            .get_element_by_id("contextMenu")
            .ok_or(VideoError::ElementNotFound("contextMenu".to_string()))?
            .query_selector_all(".context-menu-item")
            .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to get playback speed button: {:?}", e)))?
            .get(1)
            .ok_or(VideoError::ElementNotFound("playback speed button".to_string()))?;
            
        playback_speed_button.add_event_listener_with_callback(
            "click",
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
            .get_element_by_id("contextMenu")
            .ok_or(VideoError::ElementNotFound("contextMenu".to_string()))?
            .query_selector_all(".context-menu-item")
            .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to get pip button: {:?}", e)))?
            .get(2)
            .ok_or(VideoError::ElementNotFound("pip button".to_string()))?;
            
        pip_button.add_event_listener_with_callback(
            "click",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    // Speed options click event listeners
    {
        let speed_options = playback_speed_menu.query_selector_all(".speed-option")
            .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to get speed options: {:?}", e)))?;
        
        for i in 0..speed_options.length() {
            let option = speed_options.get(i)
                .ok_or_else(|| VideoError::ElementNotFound("Failed to get speed option".to_string()))?
                .dyn_into::<web_sys::Element>()
                .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to convert Node to Element: {:?}", e)))?;
            
            let text_content = option.text_content()
                .ok_or_else(|| VideoError::VideoOperationFailed("No text content found".to_string()))?;
            
            let speed = text_content.replace('x', "").parse::<f64>()
                .map_err(|e| VideoError::VideoOperationFailed(format!("Failed to parse speed: {:?}", e)))?;
            
            let closure = Closure::wrap(Box::new(move || {
                set_playback_speed(speed).unwrap_or_default();
            }) as Box<dyn FnMut()>);
            
            option.add_event_listener_with_callback(
                "click",
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
            .get_element_by_id("menuButton")
            .ok_or(VideoError::ElementNotFound("menuButton".to_string()))?;
            
        menu_button.add_event_listener_with_callback(
            "click",
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    Ok(())
} 