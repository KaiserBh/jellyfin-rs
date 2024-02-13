use crate::err::JellyfinError;

pub async fn handle_http_error(resp: reqwest::Response) -> JellyfinError {
    let status_code = resp.status().as_u16();
    let body = resp.text().await.unwrap_or_default();

    if let Ok(parsed_body) = serde_json::from_str::<serde_json::Value>(&body) {
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
            property1: parsed_body
                .get("property1")
                .and_then(|v| v.as_str())
                .map(String::from),
            property2: parsed_body
                .get("property2")
                .and_then(|v| v.as_str())
                .map(String::from),
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
            property1: None,
            property2: None,
        }
    }
}
