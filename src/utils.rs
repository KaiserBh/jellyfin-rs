use std::collections::HashMap;

use reqwest::Response;
use serde_json::Value;

use crate::err::JellyfinError;

pub async fn handle_http_error(resp: Response) -> JellyfinError {
    let status_code = resp.status().as_u16();
    let body = resp.text().await.unwrap_or_default();

    // Attempt to parse the body as JSON
    if let Ok(parsed_body) = serde_json::from_str::<Value>(&body) {
        // Initialize an empty HashMap for errors
        let mut errors: HashMap<String, Vec<String>> = HashMap::new();

        // Check if the "errors" field exists and is an object
        if let Some(errs) = parsed_body.get("errors").and_then(|v| v.as_object()) {
            for (key, value) in errs {
                // Assuming each key in "errors" maps to an array of strings
                if let Some(msgs) = value.as_array() {
                    errors.insert(
                        key.clone(),
                        msgs.iter()
                            .filter_map(|m| m.as_str().map(String::from))
                            .collect(),
                    );
                }
            }
        }

        JellyfinError::HttpRequestError {
            status: status_code,
            type_: parsed_body
                .get("type")
                .and_then(|v| v.as_str())
                .map(String::from),
            title: parsed_body
                .get("title")
                .and_then(|v| v.as_str())
                .map(String::from),
            detail: parsed_body
                .get("detail")
                .and_then(|v| v.as_str())
                .map(String::from),
            instance: parsed_body
                .get("instance")
                .and_then(|v| v.as_str())
                .map(String::from),
            errors, // Pass the parsed errors HashMap
            message: body,
        }
    } else {
        // Fallback to a simple error message if the response is not JSON or cannot be parsed
        JellyfinError::HttpRequestError {
            status: status_code,
            message: body,
            type_: None,
            title: None,
            detail: None,
            instance: None,
            errors: HashMap::new(),
        }
    }
}
