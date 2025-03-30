use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;

// Store mock responses
static MOCK_RESPONSES: Lazy<Mutex<HashMap<String, MockResponse>>> = 
    Lazy::new(|| {
        let mut map = HashMap::new();
        // Add default mock for greet endpoint
        map.insert(
            "/api/greet?name=World".to_string(),
            MockResponse {
                status: 200,
                body: "Hello, World!".to_string(),
            },
        );
        Mutex::new(map)
    });

#[derive(Clone)]
pub struct MockResponse {
    status: u16,
    body: String,
}

#[wasm_bindgen]
pub struct MockHttpClient;

impl MockHttpClient {
    pub fn mock_response(url: &str, status: u16, body: String) {
        let mock_response = MockResponse {
            status,
            body,
        };
        MOCK_RESPONSES.lock()
            .unwrap()
            .insert(url.to_string(), mock_response);
    }

    pub fn clear_mocks() {
        MOCK_RESPONSES.lock().unwrap().clear();
    }

    pub async fn get(&self, url: &str) -> Result<String, JsValue> {
        // Special handling for greet endpoint
        if url.starts_with("/api/greet?name=") {
            let name = url.split("name=").nth(1).unwrap_or("World");
            return Ok(format!("Hello, {}!", name));
        }

        if let Some(response) = MOCK_RESPONSES.lock().unwrap().get(url) {
            if response.status == 200 {
                Ok(response.body.clone())
            } else {
                Err(JsValue::from_str(&format!("HTTP Error: {}", response.status)))
            }
        } else {
            Err(JsValue::from_str("No mock response configured for this URL"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_mock_client() {
        let client = MockHttpClient;
        let test_url = "/api/greet?name=Test";
        let test_response = "Hello, Test!".to_string();

        MockHttpClient::mock_response(test_url, 200, test_response.clone());

        let result = client.get(test_url).await.unwrap();
        assert_eq!(result, test_response);
    }

    #[wasm_bindgen_test]
    async fn test_mock_client_error() {
        let client = MockHttpClient;
        let test_url = "/api/Error?name=Error";
        
        MockHttpClient::mock_response(test_url, 404, "Not Found".to_string());

        let result = client.get(test_url).await;
        assert!(result.is_err());
    }
} 