use crate::{
    tests::{get_config, init_test_client},
    JellyfinClient,
};

#[tokio::test]
async fn test_get_user() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    // TODO: Finish writing this test.

    Ok(())
}

#[tokio::test]
async fn auth_user_name_success() -> Result<(), Box<dyn std::error::Error>> {
    let (server_url, username, password) = get_config();

    let mut client = JellyfinClient::new(server_url).await?;
    client.auth_user_name(username, password).await?;

    // Assert that the client's `auth` field is now set
    assert!(
        client.auth.is_some(),
        "Client auth should be set after successful authentication"
    );

    Ok(())
}

#[tokio::test]
async fn auth_user_name_failure() -> Result<(), Box<dyn std::error::Error>> {
    let (server_url, _, _) = get_config();

    let username = "invalid_user";
    let password = "wrong_password";

    let mut client = JellyfinClient::new(server_url).await?;

    // Attempt to authenticate with incorrect credentials
    let result = client.auth_user_name(username, password).await;

    // Check that the result is an error
    assert!(
        result.is_err(),
        "Authentication should fail with incorrect credentials"
    );

    Ok(())
}
