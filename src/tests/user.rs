use crate::{
    err::JellyfinError,
    tests::{get_config, init_test_client},
    user::SubtitleMode,
    JellyfinClient,
};

#[tokio::test]
async fn create_user_success() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    let created_user = client
        .create_user("tempuser".to_string(), "tempuser".to_string())
        .await?;

    assert_eq!(created_user.name, "tempuser".to_string());

    // Clean up
    client.delete_user(created_user.id).await?;

    Ok(())
}

#[tokio::test]
async fn create_user_duplicate_username() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    let created_user = client
        .create_user("temp".to_string(), "temp".to_string())
        .await?;

    assert_eq!(created_user.name, "temp".to_string());

    let result_duplicate_user = client
        .create_user("temp".to_string(), "temp".to_string())
        .await;

    assert!(result_duplicate_user.is_err());

    // Clean up
    client.delete_user(created_user.id).await?;

    Ok(())
}

#[tokio::test]
async fn update_user_success() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    let mut created_user = client
        .create_user("updateuser".to_string(), "updateuser".to_string())
        .await?;

    assert_eq!(created_user.name, "updateuser".to_string());
    assert_eq!(
        created_user.configuration.subtitle_mode,
        SubtitleMode::Default
    );

    created_user.name = "UpdatedNewName".to_string();
    created_user.configuration.subtitle_mode = SubtitleMode::Smart;

    let update_user = client
        .update_user(&created_user.id, created_user.clone())
        .await;

    assert!(update_user.is_ok(), "Updating user should be successful");

    // Get the updated user and assert
    let updated_user = client.get_user_by_id(created_user.id.clone()).await?;

    assert_eq!(updated_user.name, "UpdatedNewName");
    assert_eq!(
        updated_user.configuration.subtitle_mode,
        SubtitleMode::Smart
    );

    // Clean up
    client.delete_user(created_user.id).await?;

    Ok(())
}

#[tokio::test]
async fn delete_user_success() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    let created_user = client
        .create_user("deleteuser".to_string(), "deleteuser".to_string())
        .await?;

    assert_eq!(created_user.name, "deleteuser".to_string());

    // Clean up
    client.delete_user(&created_user.id).await?;

    // Confirm user is deleted

    let not_found_user = client.get_user_by_id(created_user.id).await;

    assert!(
        not_found_user.is_err(),
        "Delete user should have been not found and give error"
    );

    match not_found_user {
        Ok(_) => panic!("Expected an error for non-existing user, but got Ok."),
        Err(e) => match e {
            JellyfinError::HttpRequestError {
                status, message, ..
            } => {
                assert_eq!(status, 404, "Expected HTTP 404 error for user not found.");
                assert_eq!(
                    message,
                    "\"User not found\"".to_string(),
                    "Expected message `User not found`"
                )
            }
            _ => panic!("Expected HttpRequestError, but got a different error."),
        },
    }

    Ok(())
}

#[tokio::test]
async fn get_users_success() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    let users = client.get_users(false, false).await?;

    assert!(
        !users.is_empty(),
        "Get user should return at least a user or more"
    );

    Ok(())
}

#[tokio::test]
async fn get_user_by_id_success() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    // Fetch all users from the server.
    let users = client.get_users(true, true).await?;

    for user in &users {
        let fetched_user = client.get_user_by_id(&user.id).await?;
        assert_eq!(&user.id, &fetched_user.id, "User IDs should match");
    }

    Ok(())
}

#[tokio::test]
async fn get_user_by_id_non_existing_id() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    let non_existing_user_id = "non_existing_user_id".to_string();

    let result = client.get_user_by_id(&non_existing_user_id).await;

    match result {
        Ok(_) => panic!("Expected an error for non-existing user ID, but got Ok."),
        Err(e) => match e {
            JellyfinError::HttpRequestError { status, .. } => {
                assert_eq!(status, 400, "Expected HTTP 400 error for invalid user ID.");
            }
            _ => panic!("Expected HttpRequestError, but got a different error."),
        },
    }

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
async fn auth_user_name() -> Result<(), Box<dyn std::error::Error>> {
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
