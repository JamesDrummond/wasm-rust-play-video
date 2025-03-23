use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

#[wasm_bindgen]
pub async fn fetch_post() -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "https://jsonplaceholder.typicode.com/posts/1",
        &opts,
    )?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    
    let post: Post = serde_wasm_bindgen::from_value(json)?;
    
    Ok(format!(
        "Post #{}: {}\n\n{}",
        post.id,
        post.title,
        post.body
    ))
} 