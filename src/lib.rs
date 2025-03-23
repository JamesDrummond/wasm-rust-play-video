use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde::{Deserialize, Serialize};

mod mock_client;
use mock_client::MockHttpClient;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct Post {
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

#[wasm_bindgen]
pub async fn greet(name: &str) -> Result<String, JsValue> {
    let client = MockHttpClient;
    let url = format!("/api/greet?name={}", name);
    client.get(&url).await
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[wasm_bindgen_test]
    async fn test_greet() {
        // Set up mock response
        let test_name = "TestUser";
        let test_url = format!("/api/greet?name={}", test_name);
        let expected_response = "Hello, TestUser!";
        
        MockHttpClient::mock_response(&test_url, 200, expected_response.to_string());

        // Test the greet function
        let result = greet(test_name).await.unwrap();
        assert_eq!(result, expected_response);
    }
}
