use wasm_bindgen::prelude::*;
use crate::logger::Logger;
use crate::player::error::{show_error, hide_error, VideoError};
use crate::player::video::{get_video_element, hide_menus};

#[wasm_bindgen]
pub async fn download_video() -> Result<(), JsValue> {
    Logger::info("Entering download_video()").map_err(|e| {
        let error = VideoError::VideoOperationFailed(e.to_string());
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    hide_menus()?;
    let video_element = get_video_element()?;
    let source = video_element.query_selector("source")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to get source element: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?
        .ok_or_else(|| {
            let error = VideoError::VideoOperationFailed("No source element found".to_string());
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    let video_url = source.get_attribute("src")
        .ok_or_else(|| {
            let error = VideoError::VideoOperationFailed("No source URL found".to_string());
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;

    let window = web_sys::window().ok_or_else(|| {
        let error = VideoError::WindowNotFound;
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    let document = window.document().ok_or_else(|| {
        let error = VideoError::DocumentNotFound;
        show_error(&error.to_string()).unwrap_or_default();
        error
    })?;
    
    // Create fetch request
    let init = web_sys::RequestInit::new();
    init.set_method("GET");
    init.set_mode(web_sys::RequestMode::Cors);
    
    let request = web_sys::Request::new_with_str_and_init(&video_url, &init)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to create request: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Fetch the video
    let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to fetch video: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    let response: web_sys::Response = response.dyn_into()
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to convert response: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Get the blob
    let blob = wasm_bindgen_futures::JsFuture::from(response.blob().unwrap())
        .await
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to get blob: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    let blob: web_sys::Blob = blob.dyn_into()
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to convert blob: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Create object URL
    let url = web_sys::Url::create_object_url_with_blob(&blob)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to create object URL: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Create anchor element
    let anchor = document.create_element("a")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to create anchor element: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Set download attributes
    anchor.set_attribute("href", &url)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to set href: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    anchor.set_attribute("download", "video.mp4")
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to set download attribute: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Append to document and click
    document.body()
        .ok_or_else(|| {
            let error = VideoError::VideoOperationFailed("No body element found".to_string());
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?
        .append_child(&anchor)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to append anchor: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    // Click the anchor element
    let html_anchor = anchor.unchecked_into::<web_sys::HtmlElement>();
    html_anchor.click();
    
    // Clean up
    web_sys::Url::revoke_object_url(&url)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to revoke object URL: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    
    document.body()
        .ok_or_else(|| {
            let error = VideoError::VideoOperationFailed("No body element found".to_string());
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?
        .remove_child(&html_anchor)
        .map_err(|e| {
            let error = VideoError::VideoOperationFailed(format!("Failed to remove anchor: {:?}", e));
            show_error(&error.to_string()).unwrap_or_default();
            error
        })?;
    hide_error()?;
    Ok(())
} 