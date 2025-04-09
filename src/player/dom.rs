use web_sys::{HtmlVideoElement, HtmlSourceElement};
use wasm_bindgen::JsCast;
use crate::player::VideoError;

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

pub fn add_video_source(src: &str, type_attr: &str) -> Result<(), VideoError> {
    let window = web_sys::window().ok_or(VideoError::WindowNotFound)?;
    let document = window.document().ok_or(VideoError::DocumentNotFound)?;
    let video_element = get_video_element()?;

    let source_element = document
        .create_element("source")
        .map_err(|js_value| VideoError::VideoOperationFailed(format!("Failed to create source element: {:?}", js_value)))?
        .dyn_into::<HtmlSourceElement>()
        .map_err(|_| VideoError::VideoOperationFailed("Failed to cast element to HtmlSourceElement".to_string()))?;

    source_element.set_src(src);
    source_element.set_type(type_attr);

    video_element
        .append_child(&source_element)
        .map_err(|js_value| VideoError::VideoOperationFailed(format!("Failed to append source element: {:?}", js_value)))?;

    Ok(())
} 