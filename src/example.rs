use wasm_bindgen::prelude::*;
use crate::mock_client::MockHttpClient;
use crate::post_client::fetch_post;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn init_example() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Window not found"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("Document not found"))?;
    
    // Test the greet function
    let greeting = greet("WebAssembly").await?;
    let result_div = document.get_element_by_id("result").ok_or_else(|| JsValue::from_str("Result div not found"))?;
    result_div.set_inner_html(&format!(
        "<p>{}</p><p>2 + 3 = {}</p>",
        greeting,
        add(2, 3)
    ));

    // Set up click handler for fetch button
    let fetch_button = document.get_element_by_id("fetchPost").ok_or_else(|| JsValue::from_str("Fetch button not found"))?;
    let post_content = document.get_element_by_id("postContent").ok_or_else(|| JsValue::from_str("Post content div not found"))?;
    
    let closure = Closure::wrap(Box::new(move || {
        let post_content = post_content.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match fetch_post().await {
                Ok(post_data) => {
                    let html_content = post_data.replace('\n', "<br>");
                    post_content.set_inner_html(&html_content);
                }
                Err(error) => {
                    log(&format!("Error fetching post: {:?}", error));
                    post_content.set_inner_html("Error fetching post data");
                }
            }
        });
    }) as Box<dyn FnMut()>);
    
    fetch_button.add_event_listener_with_callback(
        "click",
        closure.into_js_value().unchecked_ref(),
    )?;

    Ok(())
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