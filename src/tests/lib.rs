use dotenv::dotenv;
use std::error::Error;

use crate::JellyfinClient;

pub fn get_config() -> (String, String, String) {
    dotenv().ok();
    let server_url = std::env::var("JF_SERVER_URL").expect("JF_SERVER_URL not set");
    let username = std::env::var("JF_USERNAME").expect("JF_USERNAME not set");
    let password = std::env::var("JF_PASSWORD").expect("JF_PASSWORD not set");

    (server_url, username, password)
}

pub async fn init_test_client() -> Result<JellyfinClient, Box<dyn Error>> {
    let (server_url, username, password) = get_config();

    let client = JellyfinClient::new_auth_name(server_url, username, password)
        .await
        .expect("Failed to auth");

    Ok(client)
}

#[tokio::test]
async fn test_new_with_valid_url() -> Result<(), Box<dyn Error>> {
    let test_url = "http://example.com";

    let result = JellyfinClient::new(test_url).await?;

    assert_eq!(
        result.url.as_str(),
        "http://example.com/",
        "URL should be correctly parsed"
    );

    assert!(result.auth.is_none(), "Auth should be None");

    Ok(())
}

#[tokio::test]
async fn test_new_with_invalid_url() {
    let test_url = "invalid_url";

    let result = JellyfinClient::new(test_url).await;

    assert!(
        result.is_err(),
        "Function should return an Err for an invalid URL"
    );
}
