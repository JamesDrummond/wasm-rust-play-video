use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

#[derive(Debug)]
pub enum VideoError {
    WindowNotFound,
    DocumentNotFound,
    ElementNotFound(String),
    VideoOperationFailed(String),
    StateError(String),
}

impl std::fmt::Display for VideoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoError::WindowNotFound => write!(f, "Window not found"),
            VideoError::DocumentNotFound => write!(f, "Document not found"),
            VideoError::ElementNotFound(id) => write!(f, "Element not found: {}", id),
            VideoError::VideoOperationFailed(msg) => write!(f, "Video operation failed: {}", msg),
            VideoError::StateError(msg) => write!(f, "State error: {}", msg),
        }
    }
}

impl std::error::Error for VideoError {}

impl From<VideoError> for JsValue {
    fn from(error: VideoError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

#[wasm_bindgen]
pub fn show_error(message: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    
    let error_element = document
        .get_element_by_id("errorMessage")
        .ok_or(VideoError::ElementNotFound("errorMessage".to_string()))?;
    
    error_element.set_text_content(Some(message));
    error_element.set_attribute("class", "error-message show")?;
    Ok(())
}

#[wasm_bindgen]
pub fn hide_error() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    
    let error_element = document
        .get_element_by_id("errorMessage")
        .ok_or(VideoError::ElementNotFound("errorMessage".to_string()))?;
    
    error_element.set_text_content(None);
    error_element.set_attribute("class", "error-message")?;
    Ok(())
} 