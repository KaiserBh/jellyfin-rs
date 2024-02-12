use super::err::Result;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::json;
use sha1::Digest;

use super::session::SessionInfo;
use crate::err::JellyfinError;
use crate::JellyfinClient;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub name: String,
    pub server_id: String,
    pub server_name: Option<String>,
    pub id: String,
    pub primary_image_tag: Option<String>,
    pub has_password: bool,
    pub has_configured_password: bool,
    pub has_configured_easy_password: bool,
    pub enable_auto_login: bool,
    pub last_login_date: Option<String>,
    pub last_activity_date: Option<String>,
    pub configuration: UserConfiguration,
    pub policy: UserPolicy,
    pub primary_image_aspect_ratio: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserConfiguration {
    pub audio_language_preference: Option<String>,
    pub play_default_audio_track: bool,
    pub subtitle_language_preference: String,
    pub display_missing_episodes: bool,
    pub grouped_folders: Vec<String>,
    pub subtitle_mode: String,
    pub display_collections_view: bool,
    pub enable_local_password: bool,
    pub ordered_views: Vec<String>,
    pub latest_items_excludes: Vec<String>,
    pub my_media_excludes: Vec<String>,
    pub hide_played_in_latest: bool,
    pub remember_audio_selections: bool,
    pub remember_subtitle_selections: bool,
    pub enable_next_episode_auto_play: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserPolicy {
    pub is_administrator: bool,
    pub is_hidden: bool,
    pub is_disabled: bool,
    pub max_parental_rating: Option<i64>,
    pub blocked_tags: Vec<String>,
    pub enable_user_preference_access: bool,
    pub access_schedules: Vec<UserAccessSchedule>,
    pub block_unrated_items: Vec<String>,
    pub enable_remote_control_of_other_users: bool,
    pub enable_shared_device_control: bool,
    pub enable_remote_access: bool,
    pub enable_live_tv_management: bool,
    pub enable_live_tv_access: bool,
    pub enable_media_playback: bool,
    pub enable_audio_playback_transcoding: bool,
    pub enable_video_playback_transcoding: bool,
    pub enable_playback_remuxing: bool,
    pub force_remote_source_transcoding: bool,
    pub enable_content_deletion: bool,
    pub enable_content_deletion_from_folders: Vec<String>,
    pub enable_content_downloading: bool,
    pub enable_sync_transcoding: bool,
    pub enable_media_conversion: bool,
    pub enabled_devices: Vec<String>,
    pub enable_all_devices: bool,
    pub enabled_channels: Vec<String>,
    pub enable_all_channels: bool,
    pub enabled_folders: Vec<String>,
    pub enable_all_folders: bool,
    pub invalid_login_attempt_count: i64,
    pub login_attempts_before_lockout: i64,
    pub max_active_sessions: i64,
    pub enable_public_sharing: bool,
    pub blocked_media_folders: Vec<String>,
    pub blocked_channels: Vec<String>,
    pub remote_client_bitrate_limit: i64,
    pub authentication_provider_id: String,
    pub password_reset_provider_id: String,
    pub sync_play_access: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserAccessSchedule {
    pub user_id: String,
    pub day_of_week: String,
    pub start_hour: i64,
    pub end_hour: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserAuth {
    pub user: User,
    pub session_info: SessionInfo,
    pub access_token: String,
    pub server_id: String,
}

impl UserAuth {
    pub fn to_emby_header(&self) -> String {
        let device_name = whoami::devicename().replace(' ', "_");

        format!("MediaBrowser Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"{}\"",  device_name, md5::compute(device_name.clone()), self.access_token)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GetUsersQuery {
    is_hidden: bool,
    is_disabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AuthUserStdQuery {
    pw: String,
    password: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AuthUserNameQuery {
    username: String,
    pw: String,
}

impl JellyfinClient {
    /// Gets a list of all users that the authenticated user has access to, given some filters.
    ///
    /// # Arguments
    ///
    /// * `is_hidden` - Filter for users that are hidden.
    /// * `is_disabled` - Filter for users that are disabled.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping a vector of `User` instances if successful, or a `JellyfinError` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///  use jellyfin_rs::JellyfinClient;
    ///
    /// async fn example_usage(client: &JellyfinClient) {
    ///     let users = client.get_users(false, false).await;
    ///     match users {
    ///         Ok(users) => println!("Found {} users.", users.len()),
    ///         Err(e) => eprintln!("Error fetching users: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_users(&self, is_hidden: bool, is_disabled: bool) -> Result<Vec<User>> {
        let endpoint_url = self.url.join("/Users").expect("Failed to join URL");

        let response = self
            .client
            .get(endpoint_url)
            .query(&GetUsersQuery {
                is_hidden,
                is_disabled,
            })
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json::<Vec<User>>()
                        .await
                        .map_err(JellyfinError::NetworkError)
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Fetches a user by their ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `User` instance if successful, or a `JellyfinError` otherwise.
    pub async fn get_user_by_id<T: Into<String>>(&self, id: T) -> Result<User> {
        let id_str = id.into();
        let endpoint_url = self
            .url
            .join(&format!("/Users/{}", id_str))
            .expect("Failed to join URL");

        let response = self
            .client
            .get(endpoint_url)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json::<User>()
                        .await
                        .map_err(JellyfinError::NetworkError)
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Deletes a user by their ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user to delete.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success if the user was successfully deleted, or a `JellyfinError` otherwise.
    pub async fn delete_user<T: Into<String>>(&self, id: T) -> Result<()> {
        let id_str = id.into();
        let endpoint_url = self
            .url
            .join(&format!("/Users/{}", id_str))
            .expect("Failed to join URL");

        let response = self
            .client
            .delete(endpoint_url)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Updates user information for a given user ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user to update.
    /// * `new_info` - The new information to update the user with.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success if the user information was successfully updated, or a `JellyfinError` otherwise.
    pub async fn update_user<T: Into<String>>(&self, id: T, new_info: User) -> Result<()> {
        let id_str = id.into();
        let endpoint_url = self
            .url
            .join(&format!("/Users/{}", id_str))
            .expect("Failed to join URL");

        let response = self
            .client
            .post(endpoint_url)
            .json(&new_info)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Authenticates a user by standard method using user ID and password.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user to authenticate.
    /// * `password` - The password of the user.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success if the user was authenticated successfully, or a `JellyfinError` otherwise.
    pub async fn auth_user_std<T: Into<String> + Clone>(
        &mut self,
        id: T,
        password: T,
    ) -> Result<()> {
        let mut hasher = sha1::Sha1::new();
        hasher.update(password.clone().into());
        let device_name = whoami::devicename().replace(' ', "_");

        let endpoint_url = self
            .url
            .join(&format!("/Users/{}/Authenticate", id.clone().into()))
            .expect("Failed to join URL");

        let response = self
        .client
        .post(endpoint_url)
        .query(&AuthUserStdQuery {
            pw: password.into(),
            password: format!("{:x}", hasher.finalize()),
        })
        .header("X-Emby-Authorization", format!("MediaBrowser Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
        .send()
        .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    self.auth = Some(resp.json().await.map_err(JellyfinError::NetworkError)?);
                    Ok(())
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Updates the configuration for a given user ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user whose configuration is to be updated.
    /// * `new_conf` - The new configuration details for the user.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success if the user's configuration was successfully updated, or a `JellyfinError` otherwise.
    pub async fn update_user_conf<T: Into<String>>(
        &self,
        id: T,
        new_conf: UserConfiguration,
    ) -> Result<()> {
        let endpoint_url = self
            .url
            .join(&format!("/Users/{}/Configuration", id.into()))
            .expect("Failed to join URL");

        let response = self
            .client
            .post(endpoint_url)
            .json(&new_conf)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Updates the password for a given user ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user whose password is to be updated.
    /// * `new_password` - The new password for the user.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success if the user's password was successfully updated, or a `JellyfinError` otherwise.
    pub async fn update_user_password<T: Into<String>>(
        &self,
        id: T,
        new_password: T,
    ) -> Result<()> {
        let endpoint_url = self
            .url
            .join(&format!("/Users/{}/Password", id.into()))
            .expect("Failed to join URL");

        let response = self
            .client
            .post(endpoint_url)
            .json(&json!({ "NewPw": new_password.into() }))
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Updates the policy for a given user ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user whose policy is to be updated.
    /// * `new_policy` - The new policy details for the user.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success if the user's policy was successfully updated, or a `JellyfinError` otherwise.
    pub async fn update_user_policy<T: Into<String>>(
        &self,
        id: T,
        new_policy: UserPolicy,
    ) -> Result<()> {
        let endpoint_url = self
            .url
            .join(&format!("/Users/{}/Policy", id.into()))
            .expect("Failed to join URL");

        let response = self
            .client
            .post(endpoint_url)
            .json(&new_policy)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Authenticates a user by their username and password.
    ///
    /// This function attempts to authenticate a user against the Jellyfin server using the provided
    /// username and password. If authentication is successful, the user's authentication token is stored
    /// within the client instance for subsequent requests.
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice or string-like object representing the username of the user.
    /// * `password` - A string slice or string-like object representing the password of the user.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Authentication was successful.
    /// * `Err(JellyfinError::NetworkError)` - An error occurred during the network request.
    /// * `Err(JellyfinError::HttpRequestError)` - The server responded with a non-success status code, potentially
    ///   indicating incorrect credentials or an issue with the Jellyfin server.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///  use jellyfin_rs::JellyfinClient;
    ///
    /// async fn authenticate_user(client: &mut JellyfinClient) {
    ///     let username = "exampleUser";
    ///     let password = "examplePassword";
    ///     match client.auth_user_name(username, password).await {
    ///         Ok(_) => println!("Authentication successful."),
    ///         Err(e) => eprintln!("Authentication failed: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// Upon successful authentication, the user's authentication token is stored in the client instance,
    /// allowing subsequent requests to be made as the authenticated user. This token should be protected,
    /// and the client instance should not be shared with untrusted code.
    pub async fn auth_user_name<T: Into<String>>(
        &mut self,
        username: T,
        password: T,
    ) -> Result<()> {
        let device_name = whoami::devicename().replace(' ', "_");

        let endpoint_url = self
            .url
            .join("/Users/AuthenticateByName")
            .expect("Failed to join URL");

        let response = self.client.post(endpoint_url)
            .json(&AuthUserNameQuery {
                username: username.into(),
                pw: password.into(),
            })
            .header("X-Emby-Authorization", format!("MediaBrowser Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    self.auth = Some(resp.json().await.map_err(JellyfinError::NetworkError)?);

                    Ok(())
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Initiates the forgot password process for a given username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the account that forgot its password.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success if the process was initiated successfully, or a `JellyfinError` otherwise.
    pub async fn user_forgot_password<T: Into<String>>(&self, username: T) -> Result<()> {
        let device_name = whoami::devicename().replace(' ', "_");
        let endpoint_url = self
            .url
            .join("/Users/ForgotPassword")
            .expect("Failed to join URL");

        let response = self.client.post(endpoint_url).json(&json!({
            "EnteredUsername": username.into()
        }))
        .header("X-Emby-Authorization", format!("MediaBrowser Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
        .send()
        .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Redeems a forgot password PIN for resetting the password.
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN received for resetting the password.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success if the PIN was redeemed successfully, or a `JellyfinError` otherwise.
    pub async fn user_redeem_forgot_password_pin<T: Into<String>>(&self, pin: T) -> Result<()> {
        let device_name = whoami::devicename().replace(' ', "_");
        let endpoint_url = self
            .url
            .join("/Users/ForgotPassword/Pin")
            .expect("Failed to join URL");

        let response = self
        .client
        .post(endpoint_url)
        .json(&json!({
            "Pin": pin.into()
        }))
        .header("X-Emby-Authorization", format!("MediaBrowser Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
        .send()
        .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Retrieves the user authenticated by the current session.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the authenticated `User` instance if successful, or a `JellyfinError` otherwise.
    pub async fn get_user_by_auth(&self) -> Result<User> {
        let endpoint_url = self.url.join("/Users/Me").expect("Failed to join URL");

        let response = self
            .client
            .get(endpoint_url)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json().await.map_err(JellyfinError::NetworkError)
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Creates a new user with the specified username and password.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for the new user.
    /// * `password` - The password for the new user.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the newly created `User` instance if successful, or a `JellyfinError` otherwise.
    pub async fn create_user<T: Into<String>>(&self, username: T, password: T) -> Result<User> {
        let endpoint_url = self.url.join("/Users/New").expect("Failed to join URL");

        let response = self
            .client
            .post(endpoint_url)
            .json(&json!({
                "Name": username.into(),
                "Password": password.into()
            }))
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json().await.map_err(JellyfinError::NetworkError)
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }

    /// Retrieves a list of public users.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping a vector of `User` instances if successful, or a `JellyfinError` otherwise.
    pub async fn get_public_user_list(&self) -> Result<Vec<User>> {
        let device_name = whoami::devicename().replace(' ', "_");
        let endpoint_url = self.url.join("/Users/Public").expect("Failed to join URL");

        let response = self.client.get(endpoint_url)
        .header("X-Emby-Authorization", format!("MediaBrowser Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
        .send()
        .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json().await.map_err(JellyfinError::NetworkError)
                } else {
                    let status_code = resp.status().as_u16();
                    let error_message = resp.text().await.unwrap_or_default();
                    Err(JellyfinError::HttpRequestError {
                        status: status_code,
                        message: error_message,
                    })
                }
            }
            Err(e) => Err(JellyfinError::NetworkError(e)),
        }
    }
}

#[cfg(test)]
#[path = "tests/user.rs"]
mod tests;
