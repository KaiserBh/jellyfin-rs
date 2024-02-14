use uuid::Uuid;

use crate::{
    err::JellyfinError,
    tests::{get_config, init_test_client},
    user::SubtitleMode,
    JellyfinClient,
};

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

#[tokio::test]
async fn create_user_success() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    let random_user_name = generate_uuid();

    let created_user = client
        .create_user(random_user_name.clone(), random_user_name.clone())
        .await?;

    assert_eq!(created_user.name, random_user_name);

    // Clean up
    client.delete_user(created_user.id).await?;

    Ok(())
}

#[tokio::test]
async fn create_user_duplicate_username() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    let random_user_name = generate_uuid();

    let created_user = client
        .create_user(random_user_name.clone(), random_user_name.clone())
        .await?;

    assert_eq!(created_user.name, random_user_name.clone());

    let result_duplicate_user = client
        .create_user(random_user_name.clone(), random_user_name.clone())
        .await;

    assert!(result_duplicate_user.is_err());

    // Clean up
    client.delete_user(created_user.id).await?;

    Ok(())
}

#[tokio::test]
async fn update_user_success() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    let random_user_name = generate_uuid();

    let mut created_user = client
        .create_user(random_user_name.clone(), random_user_name.clone())
        .await?;

    assert_eq!(created_user.name, random_user_name);
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

    let random_user_name = generate_uuid();

    let created_user = client
        .create_user(random_user_name.clone(), random_user_name.clone())
        .await?;

    assert_eq!(created_user.name, random_user_name.clone());

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
async fn auth_user_std_success() -> Result<(), Box<dyn std::error::Error>> {
    let (url, _, password) = get_config();
    let client = init_test_client().await?;

    let user_id = client.auth.clone().unwrap().user.id;

    let mut client2 = JellyfinClient::new_auth_std(url, user_id.clone(), password.clone()).await?;

    client2.auth_user_std(user_id, password).await?;

    // Assert that the client's `auth` field is now set
    assert!(
        client2.auth.is_some(),
        "Client auth should be set after successful standard authentication using id and password"
    );

    Ok(())
}

#[tokio::test]
async fn auth_user_std_user_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let (_, _, password) = get_config();

    let mut client = init_test_client().await?;

    let not_found_user_id = "16e7d2b322a84b3cae8e0b06a04c9f79".to_string();

    let result = client
        .auth_user_std(not_found_user_id.clone(), password)
        .await;

    assert_ne!(
        client.auth.unwrap().user.id,
        not_found_user_id,
        "Client auth user id should not match"
    );

    match result {
        Ok(_) => panic!("Expected an error for non-existing user ID, but got Ok."),

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

#[tokio::test]
async fn update_user_config_success() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_test_client().await?;

    let random_user_name = generate_uuid();

    let mut created_user = client
        .create_user(random_user_name.clone(), random_user_name.clone())
        .await?;

    assert_eq!(created_user.name, random_user_name.clone());
    assert_eq!(
        created_user.configuration.subtitle_mode,
        SubtitleMode::Default
    );

    created_user.configuration.subtitle_mode = SubtitleMode::Smart;
    created_user.configuration.display_missing_episodes = true;

    let update_user = client
        .update_user_conf(&created_user.id, created_user.configuration.clone())
        .await;

    assert!(update_user.is_ok(), "Updating user config be successful");

    // Get the updated user and assert
    let updated_user = client.get_user_by_id(created_user.id.clone()).await?;

    assert_eq!(
        updated_user.configuration.subtitle_mode,
        SubtitleMode::Smart,
        "Subtitle Mode should be smart after updating user config"
    );

    assert!(
        updated_user.configuration.display_missing_episodes,
        "Display missing episode should be true after config is updated"
    );

    // Clean up
    client.delete_user(created_user.id).await?;

    Ok(())
}

#[tokio::test]
async fn update_user_password_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = init_test_client().await?;

    let random_user_name = generate_uuid();

    let created_user = client
        .create_user(random_user_name.clone(), random_user_name.clone())
        .await?;

    assert_eq!(created_user.name, random_user_name.clone());

    let update_user = client
        .update_user_password(created_user.id.clone(), "newpassword".to_string())
        .await;

    assert!(
        update_user.is_ok(),
        "Updating user password should be successful"
    );

    let (url, _, _) = get_config();

    let mut client2 =
        JellyfinClient::new_auth_std(url, created_user.id.clone(), "newpassword".to_string())
            .await?;

    // Try to auth user with the new password
    let auth_user = client2
        .auth_user_std(created_user.id.clone(), "newpassword".to_string())
        .await;

    assert!(
        auth_user.is_ok(),
        "Authenticating after changing password should be successful but faile"
    );

    // Have to init client again here that way that way auth header is set to admin again.
    client = init_test_client().await?;

    // Clean Up
    client.delete_user(created_user.id).await?;

    Ok(())
}
