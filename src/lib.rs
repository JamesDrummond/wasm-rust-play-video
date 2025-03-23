use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;

mod mock_client;
mod post_client;
mod video;
use mock_client::MockHttpClient;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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
